use std::fmt::Debug;

use color_eyre::Result;
use pathfinding::directed::cycle_detection::brent;

#[derive(Clone, PartialEq, Eq)]
struct Platform {
    side_len: usize,
    cols: Vec<Vec<Stack>>,
}

impl Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = vec![vec!['.'; self.side_len]; self.side_len];
        for (i, col) in self.cols.iter().enumerate() {
            for stack in col.iter() {
                if stack.start > 0 {
                    grid[stack.start as usize - 1][i] = '#';
                }
                for j in 0..stack.round_rocks {
                    grid[stack.start as usize + j as usize][i] = 'O';
                }
            }
        }
        for row in grid.iter() {
            for c in row.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Platform {
    fn load_when_tilted_east(&self) -> u64 {
        let mut sum = 0;
        for (i, col) in self.cols.iter().enumerate() {
            let mut row_sum = 0;
            for stack in col.iter() {
                row_sum += stack.round_rocks as u64 * (self.side_len as u64 - i as u64);
            }
            sum += row_sum;
        }
        sum
    }

    // We're assuming the platform is a square
    fn rotate_tilt(self) -> Platform {
        let mut new_cols = vec![
            vec![Stack {
                start: 0,
                round_rocks: 0,
            }];
            self.side_len
        ];
        for (i, col) in self.cols.iter().enumerate() {
            for stack in col.iter() {
                if stack.start > 0 {
                    new_cols[self.side_len - stack.start as usize].push(Stack {
                        start: i as u8 + 1,
                        round_rocks: 0,
                    });
                }
                for j in 0..stack.round_rocks {
                    new_cols[self.side_len - stack.start as usize - 1 - j as usize]
                        .last_mut()
                        .unwrap()
                        .round_rocks += 1;
                }
            }
        }

        Platform {
            side_len: self.side_len,
            cols: new_cols,
        }
    }
}

impl From<&str> for Platform {
    fn from(input: &str) -> Self {
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
        Platform {
            side_len: rows,
            cols: stacks,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    let platform = Platform::from(input);
    let p1 = platform
        .cols
        .iter()
        .map(|s| {
            s.iter()
                .map(|stack| stack.load(platform.side_len as u32))
                .sum::<u32>() as u64
        })
        .sum::<u64>();

    // Finish the first cycle
    let first_platform = platform.rotate_tilt().rotate_tilt().rotate_tilt();

    let one_cyle = |p: Platform| p.rotate_tilt().rotate_tilt().rotate_tilt().rotate_tilt();

    let (cycle_size, _p, i) = brent(first_platform.clone(), one_cyle);
    let equivalent = (1_000_000_000 - i - 1) % cycle_size;
    let mut platform = first_platform;
    for _ in 0..(equivalent + i) {
        platform = one_cyle(platform);
    }
    let p2 = platform.load_when_tilted_east();
    Ok((p1, p2))
}

#[cfg(test)]
mod tests {
    use crate::runner::test::{input_test, sample_test};

    use super::*;

    sample_test!(sample_part1, 14, Some(136), None);
    sample_test!(sample_part2, 14, None, Some(64));
    input_test!(part1, 14, Some(102497), None);
    input_test!(part2, 14, None, Some(105008));
}
