use std::fmt::Debug;

use color_eyre::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::grid::Grid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let input_grid = Grid::<char>::parse(input);
    let p1 = count_energized_tiles(&input_grid, (0, 0), Dir::East);
    let mut max_top: u32 = 0;
    let mut max_bottom: u32 = 0;
    let mut max_left: u32 = 0;
    let mut max_right: u32 = 0;

    rayon::scope(|s| {
        s.spawn(|_| {
            max_top = (0..input_grid.dimensions.0)
                .into_par_iter()
                .map(|x| count_energized_tiles(&input_grid, (x, 0), Dir::South))
                .max()
                .unwrap();
        });
        s.spawn(|_| {
            max_bottom = (0..input_grid.dimensions.0)
                .into_par_iter()
                .map(|x| {
                    count_energized_tiles(&input_grid, (x, input_grid.dimensions.1 - 1), Dir::North)
                })
                .max()
                .unwrap();
        });
        s.spawn(|_| {
            max_left = (0..input_grid.dimensions.1)
                .into_par_iter()
                .map(|y| count_energized_tiles(&input_grid, (0, y), Dir::East))
                .max()
                .unwrap();
        });
        s.spawn(|_| {
            max_right = (0..input_grid.dimensions.1)
                .into_par_iter()
                .map(|y| {
                    count_energized_tiles(&input_grid, (input_grid.dimensions.0 - 1, y), Dir::West)
                })
                .max()
                .unwrap();
        });
    });
    let p2 = *[max_top, max_bottom, max_left, max_right]
        .iter()
        .max()
        .unwrap();
    Ok((p1 as u64, p2 as u64))
}

fn count_energized_tiles(grid: &Grid<char>, start_location: (usize, usize), start_dir: Dir) -> u32 {
    let (dim_x, dim_y) = grid.dimensions;
    let mut visited: Grid<u8> = Grid::new(0, dim_x, dim_y);
    let mut to_visit: Vec<((usize, usize), Dir)> = vec![(start_location, start_dir)];
    while let Some(((x, y), dir)) = to_visit.pop() {
        if x >= dim_x || y >= dim_y {
            continue;
        }
        let visited_point = visited.at_mut(x, y);

        if *visited_point & (1 << dir as u8) > 0 {
            continue;
        }
        *visited_point |= 1 << dir as u8;

        match (grid.at(x, y), dir) {
            ('-' | '.', Dir::East) if x < dim_x - 1 => to_visit.push(((x + 1, y), Dir::East)),
            ('|' | '.', Dir::South) if y < dim_y - 1 => to_visit.push(((x, y + 1), Dir::South)),
            ('-' | '.', Dir::West) if x > 0 => to_visit.push(((x - 1, y), Dir::West)),
            ('|' | '.', Dir::North) if y > 0 => to_visit.push(((x, y - 1), Dir::North)),
            ('|', Dir::East | Dir::West) => {
                if y > 0 {
                    to_visit.push(((x, y - 1), Dir::North));
                }
                if y < dim_y - 1 {
                    to_visit.push(((x, y + 1), Dir::South));
                }
            }
            ('-', Dir::North | Dir::South) => {
                if x > 0 {
                    to_visit.push(((x - 1, y), Dir::West));
                }
                if x < dim_x - 1 {
                    to_visit.push(((x + 1, y), Dir::East));
                }
            }
            ('/', Dir::North) if x < dim_x - 1 => to_visit.push(((x + 1, y), Dir::East)),
            ('/', Dir::West) if y < dim_y - 1 => to_visit.push(((x, y + 1), Dir::South)),
            ('/', Dir::South) if x > 0 => to_visit.push(((x - 1, y), Dir::West)),
            ('/', Dir::East) if y > 0 => to_visit.push(((x, y - 1), Dir::North)),
            ('\\', Dir::North) if x > 0 => to_visit.push(((x - 1, y), Dir::West)),
            ('\\', Dir::West) if y > 0 => to_visit.push(((x, y - 1), Dir::North)),
            ('\\', Dir::South) if x < dim_x - 1 => to_visit.push(((x + 1, y), Dir::East)),
            ('\\', Dir::East) if y < dim_y - 1 => to_visit.push(((x, y + 1), Dir::South)),
            _ => {}
        }
    }
    visited.iter().filter(|&&bitset| bitset > 0).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part1, 16, Some(46), None);
    sample_test!(sample_part2, 16, None, Some(51));
    input_test!(part1, 16, Some(6514), None);
    input_test!(part2, 16, None, Some(8089));
}
