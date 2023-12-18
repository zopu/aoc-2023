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

fn successors(pos: &Pos, grid: &Grid<u8>) -> Vec<(Pos, u64)> {
    let mut v = vec![];
    match pos.axis {
        Axis::Horizontal => {
            if pos.x - 3 >= 0 {
                v.push((
                    Pos {
                        x: pos.x - 3,
                        y: pos.y,
                        axis: Axis::Vertical,
                    },
                    *grid.at(pos.x as usize - 1, pos.y as usize) as u64
                        + *grid.at(pos.x as usize - 2, pos.y as usize) as u64
                        + *grid.at(pos.x as usize - 3, pos.y as usize) as u64,
                ));
            }
            if pos.x - 2 >= 0 {
                v.push((
                    Pos {
                        x: pos.x - 2,
                        y: pos.y,
                        axis: Axis::Vertical,
                    },
                    *grid.at(pos.x as usize - 1, pos.y as usize) as u64
                        + *grid.at(pos.x as usize - 2, pos.y as usize) as u64,
                ));
            }
            if pos.x > 0 {
                v.push((
                    Pos {
                        x: pos.x - 1,
                        y: pos.y,
                        axis: Axis::Vertical,
                    },
                    *grid.at(pos.x as usize - 1, pos.y as usize) as u64,
                ));
            }
            if pos.x < grid.dimensions.0 as i32 - 1 {
                v.push((
                    Pos {
                        x: pos.x + 1,
                        y: pos.y,
                        axis: Axis::Vertical,
                    },
                    *grid.at(pos.x as usize + 1, pos.y as usize) as u64,
                ));
            }
            if pos.x + 2 <= grid.dimensions.0 as i32 - 1 {
                v.push((
                    Pos {
                        x: pos.x + 2,
                        y: pos.y,
                        axis: Axis::Vertical,
                    },
                    *grid.at(pos.x as usize + 1, pos.y as usize) as u64
                        + *grid.at(pos.x as usize + 2, pos.y as usize) as u64,
                ));
            }
            if pos.x + 3 <= grid.dimensions.0 as i32 - 1 {
                v.push((
                    Pos {
                        x: pos.x + 3,
                        y: pos.y,
                        axis: Axis::Vertical,
                    },
                    *grid.at(pos.x as usize + 1, pos.y as usize) as u64
                        + *grid.at(pos.x as usize + 2, pos.y as usize) as u64
                        + *grid.at(pos.x as usize + 3, pos.y as usize) as u64,
                ));
            }
        }
        Axis::Vertical => {
            if pos.y - 3 >= 0 {
                v.push((
                    Pos {
                        x: pos.x,
                        y: pos.y - 3,
                        axis: Axis::Horizontal,
                    },
                    *grid.at(pos.x as usize, pos.y as usize - 1) as u64
                        + *grid.at(pos.x as usize, pos.y as usize - 2) as u64
                        + *grid.at(pos.x as usize, pos.y as usize - 3) as u64,
                ));
            }
            if pos.y - 2 >= 0 {
                v.push((
                    Pos {
                        x: pos.x,
                        y: pos.y - 2,
                        axis: Axis::Horizontal,
                    },
                    *grid.at(pos.x as usize, pos.y as usize - 1) as u64
                        + *grid.at(pos.x as usize, pos.y as usize - 2) as u64,
                ));
            }
            if pos.y > 0 {
                v.push((
                    Pos {
                        x: pos.x,
                        y: pos.y - 1,
                        axis: Axis::Horizontal,
                    },
                    *grid.at(pos.x as usize, pos.y as usize - 1) as u64,
                ));
            }
            if pos.y < grid.dimensions.1 as i32 - 1 {
                v.push((
                    Pos {
                        x: pos.x,
                        y: pos.y + 1,
                        axis: Axis::Horizontal,
                    },
                    *grid.at(pos.x as usize, pos.y as usize + 1) as u64,
                ));
            }
            if pos.y + 2 <= grid.dimensions.1 as i32 - 1 {
                v.push((
                    Pos {
                        x: pos.x,
                        y: pos.y + 2,
                        axis: Axis::Horizontal,
                    },
                    *grid.at(pos.x as usize, pos.y as usize + 1) as u64
                        + *grid.at(pos.x as usize, pos.y as usize + 2) as u64,
                ));
            }
            if pos.y + 3 <= grid.dimensions.1 as i32 - 1 {
                v.push((
                    Pos {
                        x: pos.x,
                        y: pos.y + 3,
                        axis: Axis::Horizontal,
                    },
                    *grid.at(pos.x as usize, pos.y as usize + 1) as u64
                        + *grid.at(pos.x as usize, pos.y as usize + 2) as u64
                        + *grid.at(pos.x as usize, pos.y as usize + 3) as u64,
                ));
            }
        }
    }
    v
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
