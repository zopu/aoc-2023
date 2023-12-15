use color_eyre::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u8},
    combinator::value,
    IResult,
};

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Clone, Debug)]
struct LensBox {
    lenses: Vec<Lens>,
}

impl LensBox {
    pub fn new() -> Self {
        Self { lenses: Vec::new() }
    }

    pub fn add(&mut self, lens_to_add: Lens) {
        let lens = self
            .lenses
            .iter_mut()
            .find(|l| l.label == lens_to_add.label);
        if let Some(lens) = lens {
            lens.focal_length = lens_to_add.focal_length;
        } else {
            self.lenses.push(lens_to_add);
        }
    }

    pub fn remove(&mut self, label: &str) {
        self.lenses.retain(|l| l.label != label);
    }

    pub fn power(&self) -> u64 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, l)| (i + 1) as u64 * l.focal_length as u64)
            .sum()
    }
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let p1: u64 = input.split(',').map(|s| hash(0, s.chars())).sum();
    Ok((p1, part2(input)?))
}

pub fn part2(input: &str) -> Result<u64> {
    let mut boxes: Vec<LensBox> = vec![LensBox::new(); 256];
    input
        .split(',')
        .map(|s| parse_instruction(s).unwrap().1)
        .for_each(|inst| follow_instruction(&inst, &mut boxes));

    let power = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1) as u64 * b.power())
        .sum::<u64>();
    Ok(power)
}

fn follow_instruction(inst: &Instruction, boxes: &mut [LensBox]) {
    let box_i = hash(0, inst.label.chars()) as usize;
    match inst.op {
        Op::Place(focal_length) => {
            boxes[box_i].add(Lens {
                label: inst.label.to_string(),
                focal_length,
            });
        }
        Op::Remove => {
            boxes[box_i].remove(inst.label);
        }
    }
}

#[derive(Clone, Debug)]
enum Op {
    Place(u8),
    Remove,
}

#[derive(Clone, Debug)]
struct Instruction<'a> {
    label: &'a str,
    op: Op,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (remaining, label) = alpha1(input)?;
    let (remaining, mut op) =
        alt((value(Op::Remove, tag("-")), value(Op::Place(0), tag("="))))(remaining)?;
    if let Op::Place(_) = op {
        let (_remaining, n) = u8(remaining)?;
        op = Op::Place(n);
    }
    Ok(("", Instruction { label, op }))
}

fn hash(mut current: u64, input: impl Iterator<Item = char>) -> u64 {
    for c in input {
        let ascii = c as u8;
        current += ascii as u64;
        current *= 17;
        current %= 256;
    }
    current
}

#[cfg(test)]
mod tests {
    use crate::runner::test::{input_test, sample_test};

    use super::*;

    sample_test!(sample_part1, 15, Some(1320), None);
    sample_test!(sample_part2, 15, None, Some(145));
    input_test!(part1, 15, Some(516657), None);
    input_test!(part2, 15, None, Some(210906));

    #[test]
    fn hashes_hash() {
        assert_eq!(52, hash(0, "HASH".chars()));
    }
}
