use std::{collections::HashMap, fmt::Debug};

use color_eyre::Result;

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
    let (dim_x, dim_y) = input_grid.dimensions;
    let mut visited = Grid::<HashMap<Dir, bool>>::new(
        HashMap::new(),
        input_grid.dimensions.0,
        input_grid.dimensions.1,
    );
    let mut to_visit: Vec<((usize, usize), Dir)> = vec![((0, 0), Dir::East)];
    while let Some(((x, y), dir)) = to_visit.pop() {
        if x >= dim_x || y >= dim_y {
            continue;
        }
        let visited_point = visited.at_mut(x, y);
        if visited_point.contains_key(&dir) {
            continue;
        }
        visited_point.insert(dir, true);

        match (input_grid.at(x, y), dir) {
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
    let p1 = visited.iter().filter(|hs| !hs.is_empty()).count();
    Ok((p1 as u64, 0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part1, 16, Some(46), None);
    input_test!(part1, 16, Some(6514), None);
}
