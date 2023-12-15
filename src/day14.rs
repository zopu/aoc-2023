use std::fmt::Debug;

use color_eyre::Result;
use pathfinding::directed::cycle_detection::brent;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tilt {
    East,
    West,
    North,
    South,
}

#[derive(PartialEq, Eq)]
struct Platform {
    tilt: Option<Tilt>,
    side_len: usize,
    grid: Vec<char>,
}

impl Clone for Platform {
    // I don't know why, but handrolling this is slightly (but statsig) faster than deriving Clone for Platform.
    fn clone(&self) -> Self {
        Self {
            tilt: self.tilt.clone(),
            side_len: self.side_len.clone(),
            grid: self.grid.clone(),
        }
    }
}

impl Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut it = self.grid.iter();
        for _ in 0..self.side_len {
            for _ in 0..self.side_len {
                write!(f, "{}", it.next().unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Platform {
    #[inline]
    fn grid_at(&self, row: usize, col: usize) -> &char {
        &self.grid[row * self.side_len + col]
    }

    #[allow(unused)]
    #[inline]
    fn grid_at_mut(&mut self, row: usize, col: usize) -> &mut char {
        &mut self.grid[row * self.side_len + col]
    }

    #[inline]
    fn grid_at_tilted(&self, row: usize, col: usize, tilt: &Tilt) -> &char {
        match tilt {
            Tilt::West => &self.grid[(self.side_len - col - 1) * self.side_len + row],
            Tilt::South => {
                &self.grid[(self.side_len - row - 1) * self.side_len + (self.side_len - col - 1)]
            }
            Tilt::East => &self.grid[col * self.side_len + (self.side_len - row - 1)],
            _ => &self.grid[row * self.side_len + col],
        }
    }

    #[inline]
    fn grid_at_tilted_mut(&mut self, row: usize, col: usize, tilt: &Tilt) -> &mut char {
        match tilt {
            Tilt::West => &mut self.grid[(self.side_len - col - 1) * self.side_len + row],
            Tilt::South => {
                &mut self.grid
                    [(self.side_len - row - 1) * self.side_len + (self.side_len - col - 1)]
            }
            Tilt::East => &mut self.grid[col * self.side_len + (self.side_len - row - 1)],
            _ => &mut self.grid[row * self.side_len + col],
        }
    }

    fn rotate_tilt(mut self) -> Platform {
        match self.tilt {
            None => self.tilt_north(),
            Some(Tilt::North) => self.tilt_west(),
            Some(Tilt::West) => self.tilt_south(),
            Some(Tilt::South) => self.tilt_east(),
            Some(Tilt::East) => self.tilt_north(),
        }
        self
    }

    fn cycle(self) -> Platform {
        self.rotate_tilt().rotate_tilt().rotate_tilt().rotate_tilt()
    }

    fn north_load(&self) -> u64 {
        let mut sum = 0;
        for i in 0..self.side_len {
            for j in 0..self.side_len {
                if *self.grid_at(i, j) == 'O' {
                    sum += (self.side_len - i) as u64;
                }
            }
        }
        sum
    }

    fn tilt_north(&mut self) {
        self.tilt = Some(Tilt::North);
        for col in 0..self.side_len {
            let mut place_row = 0;
            for row in 0..self.side_len {
                match self.grid[row * self.side_len + col] {
                    '#' => {
                        place_row = row + 1;
                    }
                    'O' => {
                        self.grid[row * self.side_len + col] = '.';
                        self.grid[place_row * self.side_len + col] = 'O';
                        place_row += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        self.tilt = Some(Tilt::West);
        for col in 0..self.side_len {
            let mut place_row = 0;
            for row in 0..self.side_len {
                match self.grid[(self.side_len - col - 1) * self.side_len + row] {
                    '#' => {
                        place_row = row + 1;
                    }
                    'O' => {
                        self.grid[(self.side_len - col - 1) * self.side_len + row] = '.';
                        self.grid[(self.side_len - col - 1) * self.side_len + place_row] = 'O';
                        place_row += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        self.tilt = Some(Tilt::South);
        for col in 0..self.side_len {
            let mut place_row = 0;
            for row in 0..self.side_len {
                match self.grid
                    [(self.side_len - row - 1) * self.side_len + (self.side_len - col - 1)]
                {
                    '#' => {
                        place_row = row + 1;
                    }
                    'O' => {
                        self.grid[(self.side_len - row - 1) * self.side_len
                            + (self.side_len - col - 1)] = '.';
                        self.grid[(self.side_len - place_row - 1) * self.side_len
                            + (self.side_len - col - 1)] = 'O';
                        place_row += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        self.tilt = Some(Tilt::East);
        for col in 0..self.side_len {
            let mut place_row = 0;
            for row in 0..self.side_len {
                match self.grid[col * self.side_len + (self.side_len - row - 1)] {
                    '#' => {
                        place_row = row + 1;
                    }
                    'O' => {
                        self.grid[col * self.side_len + (self.side_len - row - 1)] = '.';
                        self.grid[col * self.side_len + (self.side_len - place_row - 1)] = 'O';
                        place_row += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    fn tilt(&mut self, tilt: Tilt) {
        self.tilt = Some(tilt);
        for col in 0..self.side_len {
            let mut place_row = 0;
            for row in 0..self.side_len {
                match self.grid_at_tilted(row, col, &tilt) {
                    '#' => {
                        place_row = row + 1;
                    }
                    'O' => {
                        *self.grid_at_tilted_mut(row, col, &tilt) = '.';
                        *self.grid_at_tilted_mut(place_row, col, &tilt) = 'O';
                        place_row += 1;
                    }
                    _ => {}
                }
            }
        }
    }
}

impl From<&str> for Platform {
    fn from(input: &str) -> Self {
        let side_len = input.lines().next().unwrap().len();
        let grid = input.chars().filter(|c| *c != '\n').collect();
        Platform {
            tilt: None,
            side_len,
            grid,
        }
    }
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let platform = Platform::from(input);
    let platform = platform.rotate_tilt();
    let p1 = platform.north_load();

    // Finish the first cycle
    let first_platform = platform.rotate_tilt().rotate_tilt().rotate_tilt();

    let (cycle_size, mut p, i) = brent(first_platform.clone(), |p| p.cycle());
    let equivalent = (1_000_000_000 - i - 1) % cycle_size;
    for _ in 0..(equivalent) {
        p = p.cycle();
    }
    let p2 = p.north_load();
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
