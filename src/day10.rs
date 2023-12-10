use std::{collections::HashSet, fmt::Debug};

use color_eyre::Result;

struct Grid {
    grid: Vec<char>,
    dimensions: (usize, usize),
    start: (i32, i32),
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> char {
        self.grid[y as usize * self.dimensions.0 + x as usize]
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.dimensions.0 as i32 && y < self.dimensions.1 as i32
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                write!(f, "{}", self.get(x as i32, y as i32))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

type Position = (i32, i32);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn of_pipe(pipe_char: char, pos: Position, prev: Position) -> Self {
        let dx = pos.0 - prev.0;
        let dy = pos.1 - prev.1;
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
        let start = (
            (start_pos % dimensions.0) as i32,
            (start_pos / dimensions.0) as i32,
        );
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
    let mut pipes_in_loop = HashSet::new();
    pipes_in_loop.insert(grid.start);
    let first_pipes = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .filter_map(|(dx, dy)| {
            let (x, y) = (grid.start.0 + dx, grid.start.1 + dy);
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
        pipes_in_loop.insert(next);
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
    use crate::runner::{test_input, test_sample};

    test_sample!(sample_part1, 10, Some(4), None);
    test_input!(part1, 10, Some(6923), None);
    test_sample!(sample_part2, 10, None, Some(1));
    test_input!(part2, 10, None, Some(529));

    #[test]
    fn sample_part2_complex() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/10/sample_part2.txt")?;
        let (_p1, p2) = run(&input)?;
        assert_eq!(8, p2);
        Ok(())
    }

    #[test]
    fn sample_part2_2() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/10/sample_part2_2.txt")?;
        let (_p1, p2) = run(&input)?;
        assert_eq!(4, p2);
        Ok(())
    }

    #[test]
    fn sample_part2_3() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/10/sample_part2_3.txt")?;
        let (_p1, p2) = run(&input)?;
        assert_eq!(4, p2);
        Ok(())
    }

    #[test]
    fn sample_part2_4() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/10/sample_part2_4.txt")?;
        let (_p1, p2) = run(&input)?;
        assert_eq!(8, p2);
        Ok(())
    }
}
