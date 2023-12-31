use std::fmt::Debug;

use color_eyre::Result;

struct Grid {
    grid: Vec<char>,
    dimensions: (usize, usize),
    start: (usize, usize),
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> char {
        self.grid[y * self.dimensions.0 + x]
    }

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.dimensions.0 && y < self.dimensions.1
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

type Position = (usize, usize);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn of_pipe(pipe_char: char, pos: Position, prev: Position) -> Self {
        let dx = pos.0 as i32 - prev.0 as i32;
        let dy = pos.1 as i32 - prev.1 as i32;
        match (pipe_char, dx, dy) {
            ('|', 0, 1) | ('7', 1, 0) | ('F', -1, 0) => Self::Down,
            ('|', 0, -1) | ('L', -1, 0) | ('J', 1, 0) => Self::Up,
            ('-', 1, 0) | ('L', 0, 1) | ('F', 0, -1) => Self::Right,
            ('-', -1, 0) | ('J', 0, 1) | ('7', 0, -1) => Self::Left,
            _ => panic!("Invalid direction {},{}", dx, dy),
        }
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut start_pos = 0;
        let grid: Vec<char> = input
            .chars()
            .filter(|c| *c != '\n')
            .enumerate()
            .map(|(i, c)| {
                if c == 'S' {
                    start_pos = i;
                }
                c
            })
            .collect();
        let dimensions = (input.lines().next().unwrap().len(), input.lines().count());
        let start = ((start_pos % dimensions.0), (start_pos / dimensions.0));
        Self {
            grid,
            dimensions,
            start,
        }
    }
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let grid = Grid::from(input);
    // Find the adjacent pipes with the correct orientation
    let first_pipes = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .filter_map(|(dx, dy)| {
            let (x, y) = (grid.start.0 as i32 + dx, grid.start.1 as i32 + dy);
            if x < 0 || y < 0 {
                return None;
            }
            let (x, y) = (x as usize, y as usize);
            if grid.in_bounds(x, y) {
                match (dx, dy, grid.get(x, y)) {
                    (0, 1, '|' | 'J' | 'L') => Some((x, y)),
                    (1, 0, '-' | 'J' | '7') => Some((x, y)),
                    (0, -1, '|' | '7' | 'F') => Some((x, y)),
                    (-1, 0, '-' | 'F' | 'L') => Some((x, y)),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(2, first_pipes.len());
    let (mut next, mut prev) = (first_pipes[0], grid.start);

    let mut count = 1;
    let mut shoelace_sum: i64 = 0;
    while next != grid.start {
        shoelace_sum += prev.0 as i64 * next.1 as i64 - prev.1 as i64 * next.0 as i64;
        count += 1;
        (next, prev) = (follow_pipe(next, prev, grid.get(next.0, next.1)), next);
    }
    shoelace_sum += prev.0 as i64 * next.1 as i64 - prev.1 as i64 * next.0 as i64;
    // Pick's formula
    let p2 = shoelace_sum.unsigned_abs() / 2 - (count as u64 / 2) + 1;

    Ok((count as u64 / 2_u64, p2))
}

fn follow_pipe(pos: Position, prev: Position, pipe_char: char) -> Position {
    match Direction::of_pipe(pipe_char, pos, prev) {
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0 - 1, pos.1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{file_test, input_test, sample_test};

    sample_test!(sample_part1, 10, Some(4), None);
    input_test!(part1, 10, Some(6923), None);
    sample_test!(sample_part2, 10, None, Some(1));
    input_test!(part2, 10, None, Some(529));

    file_test!(sample_part2_complex, 10, "sample_part2.txt", None, Some(8));
    file_test!(sample_part2_2, 10, "sample_part2_2.txt", None, Some(4));
    file_test!(sample_part2_3, 10, "sample_part2_3.txt", None, Some(4));
    file_test!(sample_part2_4, 10, "sample_part2_4.txt", None, Some(8));
}
