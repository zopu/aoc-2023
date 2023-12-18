use color_eyre::Result;
use pathfinding::directed::astar::astar;

use crate::grid::Grid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
    axis: Axis,
}

impl Pos {
    fn new(x: i32, y: i32, axis: Axis) -> Self {
        Self { x, y, axis }
    }
}

fn successors(pos: &Pos, grid: &Grid<u8>) -> Vec<(Pos, u64)> {
    let mut v = vec![];
    let (x, y) = (pos.x, pos.y);
    match pos.axis {
        Axis::Horizontal => {
            for n in 1..=3 {
                if pos.x - n >= 0 {
                    let cost = h_sum(grid, pos.x - n, pos.y, n as usize);
                    v.push((Pos::new(x - n, y, Axis::Vertical), cost));
                }
                if pos.x + n <= grid.dimensions.0 as i32 - 1 {
                    let cost = h_sum(grid, pos.x + 1, pos.y, n as usize);
                    v.push((Pos::new(x + n, y, Axis::Vertical), cost));
                }
            }
        }
        Axis::Vertical => {
            for n in 1..=3 {
                if pos.y - n >= 0 {
                    let cost = v_sum(grid, pos.x, pos.y - n, n as usize);
                    v.push((Pos::new(x, y - n, Axis::Horizontal), cost));
                }
                if pos.y + n <= grid.dimensions.1 as i32 - 1 {
                    let cost = v_sum(grid, pos.x, pos.y + 1, n as usize);
                    v.push((Pos::new(x, y + n, Axis::Horizontal), cost));
                }
            }
        }
    }
    v
}

fn v_sum(grid: &Grid<u8>, x: i32, y: i32, window: usize) -> u64 {
    (0..window)
        .map(|n| *grid.at(x as usize, y as usize + n) as u64)
        .sum::<u64>()
}

fn h_sum(grid: &Grid<u8>, x: i32, y: i32, window: usize) -> u64 {
    (0..window)
        .map(|n| *grid.at(x as usize + n, y as usize) as u64)
        .sum::<u64>()
}

fn astar_heuristic(pos: &Pos, grid: &Grid<u8>) -> u64 {
    // Just assume that the most direct path only has "1"s
    let (dim_x, dim_y) = grid.dimensions;
    (dim_x as u64 - 1 - pos.x as u64) + (dim_y as u64 - 1 - pos.y as u64)
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let grid: Grid<u8> = Grid::parse(input, |c: char| c.to_string().parse::<u8>().unwrap());
    let (dim_x, dim_y) = grid.dimensions;

    let start_pos = Pos {
        x: 0,
        y: 0,
        axis: Axis::Horizontal,
    };
    let h = |pos: &Pos| astar_heuristic(pos, &grid);
    let success = |pos: &Pos| pos.x == dim_x as i32 - 1 && pos.y == dim_y as i32 - 1;
    let result = astar(&start_pos, |p| successors(p, &grid), h, success);
    let mut p1 = 0;
    if let Some((_, cost)) = result {
        p1 = cost;
    }

    let start_pos = Pos {
        x: 0,
        y: 0,
        axis: Axis::Vertical,
    };
    let result = astar(&start_pos, |p| successors(p, &grid), h, success);
    if let Some((_, cost)) = result {
        if cost < p1 {
            p1 = cost;
        }
    }
    Ok((p1, 0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part1, 17, Some(102), None);
    input_test!(part1, 17, Some(1155), None);
}
