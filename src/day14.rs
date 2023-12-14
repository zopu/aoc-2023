use color_eyre::Result;

#[derive(Clone, Debug)]
struct Stack {
    start: u8,
    round_rocks: u8,
}

impl Stack {
    fn load(&self, rows: u32) -> u32 {
        let n = self.round_rocks as u32;
        if n == 0 {
            return 0;
        }
        rows * n - n * self.start as u32 - n * (n - 1) / 2
    }
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let cols = input.lines().next().unwrap().len();
    let mut stacks: Vec<Vec<Stack>> = vec![
        vec![Stack {
            start: 0,
            round_rocks: 0,
        }];
        cols
    ];
    let mut rows = 0;
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                stacks[j].push(Stack {
                    start: i as u8 + 1,
                    round_rocks: 0,
                });
            }
            if c == 'O' {
                stacks[j].last_mut().unwrap().round_rocks += 1;
            }
        }
        rows += 1;
    }
    let sum = stacks
        .iter()
        .map(|s| s.iter().map(|stack| stack.load(rows as u32)).sum::<u32>() as u64)
        .sum::<u64>();

    Ok((sum as u64, 0))
}

#[cfg(test)]
mod tests {
    use crate::runner::test::{input_test, sample_test};

    use super::*;

    sample_test!(sample_part1, 14, Some(136), None);
    input_test!(part1, 14, Some(102497), None);
}
