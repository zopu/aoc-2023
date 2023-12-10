use color_eyre::Result;

#[derive(Debug)]
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

type Position = (usize, usize);

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
        let start = (start_pos % dimensions.0, start_pos / dimensions.0);
        Self {
            grid,
            dimensions,
            start,
        }
    }
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let grid = Grid::from(input);
    // println!("{:?}", grid);
    // println!("{:?}", grid.get(3, 1));
    // Find the adjacent pipes with the correct orientation
    let mut pipes = [(0, 1), (1, 0), (0, -1), (-1, 0)]
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
        .map(|pos| (grid.start, pos))
        .collect::<Vec<_>>();
    assert_eq!(2, pipes.len());
    let mut count = 1;
    loop {
        count += 1;
        // println!(
        //     "Following pipe A {} at {:?}",
        //     grid.get(pipes[0].1 .0, pipes[0].1 .1),
        //     pipes[0]
        // );
        // println!(
        //     "Following pipe B {} at {:?}",
        //     grid.get(pipes[1].1 .0, pipes[1].1 .1),
        //     pipes[1]
        // );
        pipes = pipes
            .iter()
            .map(|p| (p.1, follow_pipe(p.1, p.0, grid.get(p.1 .0, p.1 .1))))
            .collect();

        // println!("{:?}", pipes);
        if pipes[0].1 == pipes[1].1 {
            // println!("Found meeting point at {:?} at count {}", pipes[0].1, count);
            break;
        }
        if pipes[0].1 == pipes[1].0 {
            count -= 1;
            break;
        }
    }
    Ok((count as u64, 0))
}

fn follow_pipe(pos: Position, prev: Position, pipe_char: char) -> Position {
    let dx = pos.0 as i32 - prev.0 as i32;
    let dy = pos.1 as i32 - prev.1 as i32;
    match (pipe_char, dx, dy) {
        // Going Down
        ('|', 0, 1) | ('7', 1, 0) | ('F', -1, 0) => (pos.0, pos.1 + 1),
        // Going Up
        ('|', 0, -1) | ('L', -1, 0) | ('J', 1, 0) => (pos.0, pos.1 - 1),
        // Going Right
        ('-', 1, 0) | ('L', 0, 1) | ('F', 0, -1) => (pos.0 + 1, pos.1),
        // Going Left
        ('-', -1, 0) | ('J', 0, 1) | ('7', 0, -1) => (pos.0 - 1, pos.1),
        _ => panic!("Invalid pipe {},{},{}", pipe_char, dx, dy),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::{test_input, test_sample};

    test_sample!(sample_part1, 10, Some(4), None);
    test_input!(part1, 10, Some(6923), None);
}
