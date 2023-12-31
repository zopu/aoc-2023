use std::collections::{BTreeMap, VecDeque};

use color_eyre::{eyre::anyhow, Result};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, combinator::value,
    sequence::tuple, IResult,
};
use num::Integer;

use crate::symbol_table::SymbolTable;

#[derive(Debug)]
struct Pulse {
    sender: ModuleId,
    receiver: ModuleId,
    pulse_type: PulseType,
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let (mut modules, _module_ids, broadcaster_id, rx_id) = build_modules(input)?;

    let (mut low_counts, mut high_counts) = (0, 0);
    for _ in 0..1000 {
        let (lc, hc) = push_button(&mut modules, broadcaster_id);
        low_counts += lc;
        high_counts += hc;
    }
    let p1 = low_counts * high_counts;

    let p2 = if rx_id > 0 {
        part2(&modules, broadcaster_id)
    } else {
        0
    };

    Ok((p1, p2))
}

fn push_button(modules: &mut [Module], broadcaster_id: ModuleId) -> (u64, u64) {
    // We're looking for cycles in the four modules that feed into rx
    let (mut low_counts, mut high_counts) = (0, 0);
    let mut queue: VecDeque<Pulse> = VecDeque::new();
    queue.push_back(Pulse {
        sender: broadcaster_id, // Not bothering to model the button
        receiver: broadcaster_id,
        pulse_type: PulseType::Low,
    });

    while !queue.is_empty() {
        // println!("Queue: {:?}", &queue);
        let pulse = queue.pop_front().unwrap();
        match pulse.pulse_type {
            PulseType::High => high_counts += 1,
            PulseType::Low => low_counts += 1,
        }
        let receiver = &mut modules[pulse.receiver as usize];
        match receiver.module_type {
            ModuleType::BroadCaster => {
                // Broadcast to all outputs
                for output in &receiver.outputs {
                    queue.push_back(Pulse {
                        sender: pulse.receiver,
                        receiver: *output,
                        pulse_type: pulse.pulse_type.clone(),
                    });
                }
            }
            ModuleType::FlipFlop => {
                // Flip the output
                if let PulseType::High = pulse.pulse_type {
                    continue;
                }
                receiver.is_on = !receiver.is_on;
                let pulse_type = match receiver.is_on {
                    true => PulseType::High,
                    false => PulseType::Low,
                };
                for output in &receiver.outputs {
                    queue.push_back(Pulse {
                        sender: pulse.receiver,
                        receiver: *output,
                        pulse_type: pulse_type.clone(),
                    });
                }
            }
            ModuleType::Conjunction => {
                // Update input memory
                receiver
                    .inputs
                    .insert(pulse.sender, pulse.pulse_type.clone());
                // If all inputs are high, output high
                let all_high = receiver
                    .inputs
                    .iter()
                    .all(|(_, pulse_type)| matches!(pulse_type, PulseType::High));
                let pulse_type = if all_high {
                    PulseType::Low
                } else {
                    PulseType::High
                };
                for output in &receiver.outputs {
                    queue.push_back(Pulse {
                        sender: pulse.receiver,
                        receiver: *output,
                        pulse_type: pulse_type.clone(),
                    });
                }
            }
        }
    }
    (low_counts, high_counts)
}

fn part2(modules: &[Module], broadcaster_id: ModuleId) -> u64 {
    // Eyeballing the input, we observe that the broadcaster has 4 outputs, and each of these are the start of a MOD-12 counter
    // So we iterate through each of these and determine which "bits" are high, to get the length of the counter.
    modules
        .get(broadcaster_id as usize)
        .unwrap()
        .outputs
        .iter()
        .fold(1, |p: u64, first_id| {
            let mut binary: Vec<bool> = Vec::new();
            let mut next_flip_flop_id = *first_id;
            loop {
                let outputs = &modules.get(next_flip_flop_id as usize).unwrap().outputs;
                // There should be one output going to a flip-flop, and *maybe* one output going to a conjunction
                let mut bit_on = false;
                next_flip_flop_id = 0;
                for o in outputs {
                    let module = modules.get(*o as usize).unwrap();
                    match module.module_type {
                        ModuleType::FlipFlop => {
                            next_flip_flop_id = *o;
                        }
                        ModuleType::Conjunction => {
                            bit_on = true;
                        }
                        _ => panic!("Should never get here"),
                    }
                }
                if bit_on {
                    binary.push(true);
                } else {
                    binary.push(false);
                }
                if next_flip_flop_id == 0 {
                    break;
                }
            }
            let cycle_len = binary
                .iter()
                .enumerate()
                .filter(|(_, b)| **b)
                .map(|(i, _)| 2u64.pow(i as u32))
                .sum::<u64>();
            p.lcm(&cycle_len)
        })
}

// Last return is id of broadcaster
fn build_modules(input: &str) -> Result<(Vec<Module>, SymbolTable, ModuleId, ModuleId)> {
    let mut module_ids = SymbolTable::new();
    let mut broadcaster_id = 0;
    let mut rx_id = 0;
    // println!("digraph g {{");
    let parsed: Vec<_> = input
        .lines()
        .map(|l| {
            let (_, pl) = parse_line(l).map_err(|e| anyhow!("Parse error: {:?}", e))?;
            let mod_id = module_ids.get(pl.name);
            if let ModuleType::BroadCaster = pl.module_type {
                broadcaster_id = mod_id;
            }
            if pl.name == "rx" {
                rx_id = mod_id;
            }
            for output in &pl.outputs {
                // Populate symbol table
                let output_id = module_ids.get(output);
                if output == &"rx" {
                    rx_id = output_id;
                }
            }
            // Print dotfile format
            // match pl.module_type {
            //     ModuleType::BroadCaster => print!("{}", pl.name),
            //     ModuleType::FlipFlop => print!("{} [shape=box]", pl.name),
            //     ModuleType::Conjunction => print!("{} [shape=circle]", pl.name),
            // }
            // if !pl.outputs.is_empty() {
            //     for o in &pl.outputs {
            //         println!("{} -> {};", pl.name, o);
            //     }
            // }
            Ok(pl)
        })
        .collect::<Result<Vec<_>>>()?;
    // println!("}}");
    let mut modules: Vec<Module> = vec![
        Module {
            module_type: ModuleType::BroadCaster,
            is_on: false,
            inputs: BTreeMap::new(),
            outputs: vec![],
        };
        module_ids.len()
    ];
    for pl in &parsed {
        let mod_id = module_ids.get(pl.name);
        let module = &mut modules[mod_id as usize];
        module.module_type = pl.module_type;
        let output_ids = pl
            .outputs
            .iter()
            .map(|o| module_ids.get(o))
            .collect::<Vec<_>>();

        for output_id in &output_ids {
            module.outputs.push(*output_id as u8);
        }
        for output_id in &output_ids {
            modules[*output_id as usize]
                .inputs
                .insert(mod_id as u8, PulseType::Low);
        }
    }
    Ok((modules, module_ids, broadcaster_id as u8, rx_id as u8))
}

#[derive(Clone, Copy, Debug)]
enum ModuleType {
    BroadCaster,
    FlipFlop,
    Conjunction,
}

type ModuleId = u8;

#[derive(Debug, Clone)]
enum PulseType {
    High,
    Low,
}

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    is_on: bool,
    inputs: BTreeMap<ModuleId, PulseType>,
    outputs: Vec<ModuleId>,
}

#[derive(Debug)]
struct ParsedLine<'a> {
    module_type: ModuleType,
    name: &'a str,
    outputs: Vec<&'a str>,
}

// Example lines:
// %rf -> lq, tj
// broadcaster -> lp, fn, tp, zz
fn parse_line(line: &str) -> IResult<&str, ParsedLine> {
    let (remaining, (module_type, name)) = alt((
        value((ModuleType::BroadCaster, "broadcaster"), tag("broadcaster")),
        tuple((value(ModuleType::FlipFlop, tag("%")), alpha1)),
        tuple((value(ModuleType::Conjunction, tag("&")), alpha1)),
    ))(line)?;
    let (remaining, _) = tag(" -> ")(remaining)?;
    let (remaining, outputs) = nom::multi::separated_list1(tag(", "), alpha1)(remaining)?;

    Ok((
        remaining,
        ParsedLine {
            module_type,
            name,
            outputs,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{file_test, input_test, sample_test};

    sample_test!(sample_part1, 20, Some(32000000), None);
    input_test!(part1, 20, Some(743090292), None);
    input_test!(part2, 20, None, Some(241528184647003));

    file_test!(extended_part1, 20, "sample_2.txt", Some(11687500), None);
}
