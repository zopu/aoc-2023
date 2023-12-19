use std::{
    cell::RefCell,
    cmp::max,
    fmt::Debug,
    hash::{Hash, Hasher},
    rc::Rc,
};

use color_eyre::Result;
use pathfinding::directed::cycle_detection::brent;
use rustc_hash::FxHasher;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tilt {
    East,
    West,
    North,
    South,
}

#[derive(PartialEq, Eq, Hash)]
struct Platform {
    tilt: Option<Tilt>,
    side_len: usize,
    grid: Vec<char>,
}

impl Clone for Platform {
    // I don't know why, but handrolling this is slightly (but statsig) faster than deriving Clone for Platform.
    fn clone(&self) -> Self {
        Self {
            tilt: self.tilt,
            side_len: self.side_len,
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

    fn rotate_tilt(&mut self) {
        match self.tilt {
            None => self.tilt_north(),
            Some(Tilt::North) => self.tilt_west(),
            Some(Tilt::West) => self.tilt_south(),
            Some(Tilt::South) => self.tilt_east(),
            Some(Tilt::East) => self.tilt_north(),
        }
    }

    fn cycle(&mut self) {
        self.rotate_tilt();
        self.rotate_tilt();
        self.rotate_tilt();
        self.rotate_tilt();
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

#[derive(Debug, Clone)]
struct LazyPlatformHashIterator {
    platform: Rc<RefCell<Platform>>,
    hashes_and_loads: Rc<RefCell<Vec<(u64, u64)>>>,
    idx: usize,
}

impl LazyPlatformHashIterator {
    fn new(platform: Platform) -> Self {
        let mut s = FxHasher::default();
        platform.hash(&mut s);
        let hash = s.finish();
        let load = platform.north_load();
        let hashes_and_loads = Rc::new(RefCell::new(vec![(hash, load)]));
        let platform = Rc::new(RefCell::new(platform));
        Self {
            platform: platform.clone(),
            hashes_and_loads,
            idx: 0,
        }
    }

    fn cycle(&mut self) {
        let gap = self.hashes_and_loads.borrow().len() as i32 - self.idx as i32;
        if gap < 1 {
            self.calc_forward(1 - gap as usize);
        }
        self.idx += 1;
    }
}

impl LazyPlatformHashIterator {
    fn calc_forward(&self, n: usize) {
        for _ in 0..n {
            let mut platform = self.platform.borrow_mut();
            platform.cycle();
            let mut s = FxHasher::default();
            platform.hash(&mut s);
            let hash = s.finish();
            let load = platform.north_load();
            {
                self.hashes_and_loads.borrow_mut().push((hash, load));
            }
        }
    }
}

impl PartialEq for LazyPlatformHashIterator {
    fn eq(&self, other: &Self) -> bool {
        // Wind forward until both idxes are within the hashes_and_loads
        if max(self.idx, other.idx) >= self.hashes_and_loads.borrow().len() {
            let n = max(self.idx, other.idx) - self.hashes_and_loads.borrow().len() + 1;
            self.calc_forward(n);
        }

        self.hashes_and_loads.borrow()[self.idx].0 == other.hashes_and_loads.borrow()[other.idx].0
    }
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let mut platform = Platform::from(input);
    platform.rotate_tilt();
    let p1 = platform.north_load();

    // Finish the first cycle
    let mut first_platform = platform.clone();
    for _ in 0..3 {
        first_platform.rotate_tilt();
    }

    let it = LazyPlatformHashIterator::new(first_platform);

    let (cycle_size, _, i) = brent(it.clone(), |mut it| {
        it.cycle();
        it
    });
    let equivalent = (1_000_000_000 - i - 1) % cycle_size;
    let p2 = it.hashes_and_loads.borrow()[i + equivalent].1;
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
