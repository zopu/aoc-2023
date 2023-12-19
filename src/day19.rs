use std::{collections::HashMap, ops::Range};

use color_eyre::{eyre::anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::u16, streaming::alpha1},
    error::Error,
    multi::separated_list0,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

pub fn run(input: &str) -> Result<(u64, u64)> {
    let mut symbols = new_symbol_table();
    let mut parts: Vec<[u16; 4]> = vec![];
    let mut seen_empty_line = false;
    let mut parsed_instructions = vec![];
    let mut next_id = 2;
    for l in input.lines() {
        if l.is_empty() {
            seen_empty_line = true;
            continue;
        }
        if !seen_empty_line {
            let (_, (id, instruction, nid)) = parse_instruction_line(l, &mut symbols, next_id)
                .map_err(|e| anyhow!("Parse error: {:?}", e))?;
            next_id = nid;
            parsed_instructions.push((id, instruction));
        } else {
            let (_, part) = parse_part_line(l).map_err(|e| anyhow!("Parse error: {:?}", e))?;
            parts.push(part);
        }
    }

    let mut workflows: Vec<Vec<Instruction>> = vec![vec![]; next_id as usize];
    for (id, ins) in parsed_instructions {
        workflows[id as usize] = ins;
    }
    let first = symbols["in"];
    let mut sum = 0;
    for part in parts {
        if follow_workflow(&part, &workflows, first) {
            sum += part.iter().map(|n| *n as u64).sum::<u64>();
        }
    }
    let part_ranges: [Range<u16>; 4] = [1..4001, 1..4001, 1..4001, 1..4001];
    let p2 = count_accepted_combinantions(part_ranges, &workflows, first);
    Ok((sum, p2))
}

fn follow_workflow(part: &[u16; 4], workflows: &[Vec<Instruction>], from: u16) -> bool {
    if from == 0 {
        return true;
    }
    if from == 1 {
        return false;
    }
    for ins in &workflows[from as usize] {
        match ins {
            Instruction::Goto(to) => {
                return follow_workflow(part, workflows, *to);
            }
            Instruction::Comparison {
                category,
                op,
                operand,
                target,
            } => {
                let val = part[*category as usize];
                match op {
                    Op::Gt => {
                        if val > *operand {
                            return follow_workflow(part, workflows, *target);
                        }
                    }
                    Op::Lt => {
                        if val < *operand {
                            return follow_workflow(part, workflows, *target);
                        }
                    }
                }
            }
        }
    }
    panic!("Should never get here!");
}

fn count_accepted_combinantions(
    part_ranges: [Range<u16>; 4],
    workflows: &[Vec<Instruction>],
    from: u16,
) -> u64 {
    if from == 0 {
        return part_ranges.iter().map(|r| r.len() as u64).product();
    }
    if from == 1 {
        return 0;
    }
    if part_ranges.iter().any(|r| r.is_empty()) {
        return 0;
    }
    let mut sum: u64 = 0;
    let mut filtered_part_ranges = part_ranges.clone();
    for ins in &workflows[from as usize] {
        match ins {
            Instruction::Goto(to) => {
                return sum + count_accepted_combinantions(filtered_part_ranges, workflows, *to);
            }
            Instruction::Comparison {
                category,
                op,
                operand,
                target,
            } => {
                let val_range = filtered_part_ranges[*category as usize].clone();
                match op {
                    Op::Gt => {
                        let new_range = (*operand + 1)..val_range.end;
                        let mut child_ranges = filtered_part_ranges.clone();
                        child_ranges[*category as usize] = new_range;
                        sum += count_accepted_combinantions(child_ranges, workflows, *target);
                        filtered_part_ranges[*category as usize] = val_range.start..(*operand + 1);
                    }
                    Op::Lt => {
                        let new_range = val_range.start..*operand;
                        let mut child_ranges = filtered_part_ranges.clone();
                        child_ranges[*category as usize] = new_range;
                        sum += count_accepted_combinantions(child_ranges, workflows, *target);
                        filtered_part_ranges[*category as usize] = *operand..val_range.end;
                    }
                }
            }
        }
    }
    panic!("Should never get here!");
}

#[derive(Debug, Clone)]
enum Instruction {
    Goto(u16), // 0 == Accept, 1 == Reject
    Comparison {
        category: u8,
        op: Op,
        operand: u16,
        target: u16,
    },
}

fn parse_part_line(input: &str) -> IResult<&str, [u16; 4]> {
    let (remaining, (_, x, _, m, _, a, _, s, _)) = tuple((
        tag("{x="),
        u16,
        tag(",m="),
        u16,
        tag(",a="),
        u16,
        tag(",s="),
        u16,
        tag("}"),
    ))(input)?;
    Ok((remaining, [x, m, a, s]))
}

fn parse_instruction_line<'a>(
    line: &'a str,
    symbols: &mut HashMap<String, u16>,
    next_id: u16,
) -> IResult<&'a str, (u16, Vec<Instruction>, u16)> {
    let (remaining, label) = alpha1(line)?;
    let (label_id, mut next_id) = get_symbol_id(label, symbols, next_id);
    let (remaining, parsed) = delimited(
        tag("{"),
        separated_list0(tag(","), parse_instruction),
        tag("}"),
    )(remaining)?;
    let mut instructions = vec![];
    for parsed_instr in &parsed {
        match parsed_instr {
            ParsedInstruction::Symbol(symbol) => {
                let (id, nid) = get_symbol_id(symbol, symbols, next_id);
                next_id = nid;
                instructions.push(Instruction::Goto(id));
            }
            ParsedInstruction::Comparison {
                category,
                op,
                operand,
                target,
            } => {
                let (id, nid) = get_symbol_id(target, symbols, next_id);
                next_id = nid;
                instructions.push(Instruction::Comparison {
                    category: *category,
                    op: *op,
                    operand: *operand,
                    target: id,
                });
            }
        }
    }
    Ok((remaining, (label_id, instructions, next_id)))
}

fn get_symbol_id(symbol: &str, table: &mut HashMap<String, u16>, mut next_id: u16) -> (u16, u16) {
    let id = table.entry(symbol.to_string()).or_insert_with(|| {
        let id = next_id;
        next_id += 1;
        id
    });
    (*id, next_id)
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Gt,
    Lt,
}

#[derive(Debug)]
enum ParsedInstruction<'a> {
    Symbol(&'a str),
    Comparison {
        category: u8,
        op: Op,
        operand: u16,
        target: &'a str,
    },
}

fn parse_instruction(input: &str) -> IResult<&str, ParsedInstruction> {
    if let Ok((remaining, symbol)) = alpha1::<_, Error<_>>(input) {
        if remaining.starts_with(',') || remaining.starts_with('}') {
            return Ok((remaining, ParsedInstruction::Symbol(symbol)));
        }
    }
    let (remaining, ((category_name, op, operand), target)) = separated_pair(
        tuple((alpha1, alt((tag("<"), tag(">"))), u16)),
        tag(":"),
        alpha1,
    )(input)?;
    let category = match category_name {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => {
            return Err(nom::Err::Error(Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    };
    let op = match op {
        "<" => Op::Lt,
        ">" => Op::Gt,
        _ => {
            return Err(nom::Err::Error(Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    };

    Ok((
        remaining,
        ParsedInstruction::Comparison {
            category,
            op,
            operand,
            target,
        },
    ))
}

fn new_symbol_table() -> HashMap<String, u16> {
    let mut symbols = HashMap::new();
    symbols.insert("A".to_string(), 0);
    symbols.insert("R".to_string(), 1);
    symbols
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};
    use color_eyre::Result;

    sample_test!(sample_part1, 19, Some(19114), None);
    sample_test!(sample_part2, 19, None, Some(167409079868000));
    input_test!(part1, 19, Some(374873), None);
    input_test!(part2, 19, None, Some(122112157518711));

    #[test]
    fn can_parse_instruction() -> Result<()> {
        let s = "grc{a>3883:gb,a>3753:brm,a<3710:vhv,xr}";
        let mut symbols = new_symbol_table();
        let (remaining, (id, instructions, next_symbol)) =
            parse_instruction_line(s, &mut symbols, 2)?;
        println!("symbols: {:?}", symbols);
        assert_eq!("", remaining);
        assert_eq!(4, instructions.len());
        assert_eq!(7, next_symbol);
        assert_eq!(id, symbols["grc"]);
        Ok(())
    }

    #[test]
    fn counts_all_combinations() -> Result<()> {
        let s = "in{A}";
        let (_, p2) = super::run(s)?;
        assert_eq!(4000 * 4000 * 4000 * 4000, p2);
        Ok(())
    }

    #[test]
    fn counts_all_rejects() -> Result<()> {
        let s = "in{R}";
        let (_, p2) = super::run(s)?;
        assert_eq!(0, p2);
        Ok(())
    }

    #[test]
    fn counts_half_combinations() -> Result<()> {
        let s = "in{x>2000:A,R}";
        let (_, p2) = super::run(s)?;
        assert_eq!(2000 * 4000 * 4000 * 4000, p2);
        let s = "in{x<2001:A,R}";
        let (_, p2) = super::run(s)?;
        assert_eq!(2000 * 4000 * 4000 * 4000, p2);
        let s = "in{x>2000:R,A}";
        let (_, p2) = super::run(s)?;
        assert_eq!(2000 * 4000 * 4000 * 4000, p2);
        let s = "in{x<2001:R,A}";
        let (_, p2) = super::run(s)?;
        assert_eq!(2000 * 4000 * 4000 * 4000, p2);
        Ok(())
    }
}
