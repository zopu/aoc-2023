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
    fn set(&mut self, x: i32, y: i32, c: char) {
        self.grid[y as usize * self.dimensions.0 + x as usize] = c;
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
            if x < 0 || y < 0 {
                return None;
            }
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
        .map(|pos| (grid.start, pos))
        .collect::<Vec<_>>();
    let mut pipes = first_pipes.clone();
    assert_eq!(2, pipes.len());
    pipes.iter().for_each(|p| {
        pipes_in_loop.insert(p.1);
    });
    let mut count = 1;
    loop {
        count += 1;
        pipes = pipes
            .iter()
            .map(|p| {
                let new_pos = follow_pipe(p.1, p.0, grid.get(p.1 .0, p.1 .1));
                pipes_in_loop.insert(new_pos);
                (p.1, new_pos)
            })
            .collect();
        if pipes[0].1 == pipes[1].1 {
            break;
        }
        if pipes[0].1 == pipes[1].0 {
            count -= 1;
            break;
        }
    }

    let p2 = part2(grid, first_pipes[0].1, &pipes_in_loop);
    // println!("Part 2: {}", p2);
    Ok((count as u64, p2))
}

fn part2(mut grid: Grid, first_pipe: Position, pipes_in_loop: &HashSet<Position>) -> u64 {
    // We're going to flood fill the grid with points either on the left (l) or right (r) of the loop
    // We can see that (0, 0) is not on the loop, so that will tell us which of l and r is the outside
    // Then we count the l or r chars in the grid for the answer

    // First traverse the loop and mark everything on the l or r that isn't in the loop
    let (mut pos, mut prev) = (first_pipe, grid.start);
    while grid.get(pos.0, pos.1) != 'S' {
        let pipe_char = grid.get(pos.0, pos.1);
        let l_and_r_table = [
            // pipe char, dx, dy, of current move, dx, dy of 'side' position, l or r for side position
            ('|', 0, 1, (1, 0, 'l'), (-1, 0, 'r')),
            ('|', 0, -1, (-1, 0, 'l'), (1, 0, 'r')),
            ('-', 1, 0, (0, -1, 'l'), (0, 1, 'r')),
            ('-', -1, 0, (0, 1, 'l'), (0, -1, 'r')),
            ('J', 0, 1, (1, 0, 'l'), (0, 1, 'l')),
            ('J', 1, 0, (1, 0, 'r'), (0, 1, 'r')),
            ('L', 0, 1, (0, 1, 'r'), (-1, 0, 'r')),
            ('L', -1, 0, (0, 1, 'l'), (-1, 0, 'l')),
            ('F', -1, 0, (-1, 0, 'r'), (0, -1, 'r')),
            ('F', 0, -1, (-1, 0, 'l'), (0, -1, 'l')),
            ('7', 1, 0, (0, -1, 'l'), (1, 0, 'l')),
            ('7', 0, -1, (1, 0, 'r'), (0, -1, 'r')),
        ];
        'inner: for (match_char, dx, dy, side_1, side_2) in l_and_r_table.iter() {
            if pipe_char == *match_char && dx == &(pos.0 - prev.0) && dy == &(pos.1 - prev.1) {
                for side in [side_1, side_2].iter() {
                    let (side_dx, side_dy, side_char) = side;
                    let (x, y) = (pos.0 + side_dx, pos.1 + side_dy);
                    if grid.in_bounds(x, y) && !pipes_in_loop.contains(&(x, y)) {
                        grid.set(x, y, *side_char);
                    }
                }
                break 'inner;
            }
        }

        let next = follow_pipe(pos, prev, pipe_char);
        (pos, prev) = (next, pos);
    }
    // Now we flood fill the grid until nothing changes
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..grid.dimensions.0 {
            for j in 0..grid.dimensions.1 {
                let c = grid.get(i as i32, j as i32);
                if c == 'l' || c == 'r' || c == 'X' {
                    continue;
                }
                if pipes_in_loop.contains(&(i as i32, j as i32)) {
                    grid.set(i as i32, j as i32, 'X');
                    continue;
                }
                for n in neighbours((i as i32, j as i32), &grid) {
                    let n_c = grid.get(n.0, n.1);
                    if n_c == 'l' {
                        grid.set(i as i32, j as i32, 'l');
                        changed = true;
                    } else if n_c == 'r' {
                        grid.set(i as i32, j as i32, 'r');
                        changed = true;
                    }
                }
            }
        }
    }

    // Work out whether l or r is the outside
    let inside_char = if let 'l' = grid.get(0, 0) { 'r' } else { 'l' };

    // Spot any unclassified points
    for i in 0..grid.dimensions.0 {
        for j in 0..grid.dimensions.1 {
            let c = grid.get(i as i32, j as i32);
            if c != 'X' && c != 'l' && c != 'r' && c != 'O' {
                println!("Unclassified point at {:?} {}", (i, j), c);
            }
        }
    }

    // Count the inside chars
    grid.grid.iter().filter(|c| **c == inside_char).count() as u64
}

fn follow_pipe(pos: Position, prev: Position, pipe_char: char) -> Position {
    match Direction::of_pipe(pipe_char, pos, prev) {
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0 - 1, pos.1),
    }
}

fn neighbours(pos: Position, grid: &Grid) -> Vec<Position> {
    let mut neighbours = Vec::new();
    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
        let (x, y) = (pos.0 + dx, pos.1 + dy);
        if x < 0 || y < 0 {
            continue;
        }
        if grid.in_bounds(x, y) {
            neighbours.push((x, y));
        }
    }
    neighbours
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
