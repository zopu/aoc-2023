use std::collections::VecDeque;

use color_eyre::Result;

use crate::grid::Grid;

pub fn run(input: &str) -> Result<(u64, u64)> {
    let grid = Grid::parse(input, |c| if c == 'S' { b'O' } else { c as u8 });

    let mut min_distance = Grid::<i16>::new(-1, grid.dimensions.0, grid.dimensions.1);

    // Queue elements (x, y, distance)
    let mut bfs_queue: VecDeque<(usize, usize, usize)> = VecDeque::new();

    bfs_queue.push_back((grid.dimensions.0 / 2, grid.dimensions.1 / 2, 0));
    while !bfs_queue.is_empty() {
        let (x, y, dist) = bfs_queue.pop_front().unwrap();
        if *min_distance.at(x, y) != -1 {
            continue;
        }
        if *grid.at(x, y) == b'#' {
            continue;
        }
        *min_distance.at_mut(x, y) = dist as i16;
        for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || ny < 0 || nx >= grid.dimensions.0 as i32 || ny >= grid.dimensions.1 as i32
            {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if *min_distance.at(nx, ny) == -1 && *grid.at(nx, ny) != b'#' {
                bfs_queue.push_back((nx, ny, dist + 1));
            }
        }
    }
    let p1 = min_distance
        .iter()
        .filter(|d| **d != -1 && **d <= 64 && **d % 2 == 0)
        .count() as u64;

    let steps = 26501365;
    // Assume the grid is square
    // println!("Steps: {}", steps);
    // println!("Extra: {}", extra);

    // Looking at the input, we know the result is going to be a large diamond.
    // Most of the inside of the diamond will be "complete" grids, with alternating
    // parity. The outside of the diamond will be composed of grids partially filled
    // from the corners.

    // We also know that the resulting diamond will have an edge from corner-to-corner
    // i.e. the same as part 1's edge PLUS ONE STEP (ffs).

    let extra = steps % grid.dimensions.0;
    assert_eq!(65, extra); // for p2
    let width_tiles = ((steps - extra) / grid.dimensions.0) as u64;
    // println!("Extra: {} spaces", extra);
    // println!("Width: {} tiles", width_tiles);

    let full_tile_odd = min_distance
        .iter()
        .filter(|d| **d != -1 && **d % 2 == 1)
        .count() as u64;
    let full_tile_even = min_distance
        .iter()
        .filter(|d| **d != -1 && **d % 2 == 0)
        .count() as u64;
    // println!("Full tile odd: {}", full_tile_odd);
    // println!("Full tile even: {}", full_tile_even);

    let four_corners_odd = min_distance
        .iter()
        .filter(|d| **d % 2 == 1 && **d > extra as i16)
        .count() as u64;
    // println!("extra {}", extra as i16);
    let four_corners_even = min_distance
        .iter()
        .filter(|d| **d % 2 == 0 && **d > extra as i16)
        .count() as u64;
    // println!("Four corners odd: {}", four_corners_odd);
    // println!("Four corners even: {}", four_corners_even);

    let mut p2: u64 = ((width_tiles + 1) * (width_tiles + 1)) * full_tile_odd
        + (width_tiles * width_tiles) * full_tile_even
        - ((width_tiles + 1) * four_corners_odd)
        + width_tiles * four_corners_even;

    // Print the points reachable in 65 steps
    let mut print_grid = Grid::new(b'.', grid.dimensions.0, grid.dimensions.1);
    for i in 0..grid.dimensions.0 {
        for j in 0..grid.dimensions.1 {
            if *min_distance.at(i, j) != -1 && *min_distance.at(i, j) <= 65 {
                *print_grid.at_mut(i, j) = b'O';
            }
            if *grid.at(i, j) == b'#' {
                *print_grid.at_mut(i, j) = b'#';
            }
        }
    }
    // println!("{}", print_grid);

    // We have one spot in the diamond that is unreachable in 65 steps from center but reachable within 64 steps from corners
    // so sub width from result
    p2 -= width_tiles;

    Ok((p1, p2))
}

#[allow(unused)]
fn step(from: &Grid<u8>) -> Grid<u8> {
    let mut to = from.clone();

    for y in 0..from.dimensions.1 {
        'grid_iter: for x in 0..from.dimensions.0 {
            if *to.at(x, y) == b'#' {
                continue;
            }
            *to.at_mut(x, y) = b'.';
            for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0
                    || ny < 0
                    || nx >= from.dimensions.0 as i32
                    || ny >= from.dimensions.1 as i32
                {
                    continue;
                }
                if *from.at(nx as usize, ny as usize) == b'O' {
                    *to.at_mut(x, y) = b'O';
                    continue 'grid_iter;
                }
            }
        }
    }

    to
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::input_test;

    input_test!(part1, 21, Some(3591), None);

    input_test!(part2, 21, None, Some(598044246091826));
}
