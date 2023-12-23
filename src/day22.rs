use std::collections::BTreeSet;

use color_eyre::eyre::anyhow;
use color_eyre::Result;
use nom::sequence::tuple;
use nom::{bytes::complete::tag, IResult};

use nom::character::complete::u16 as nu16;

use crate::grid::Grid;

pub fn run(input: &str) -> Result<(u64, u64)> {
    let (mut dim_x, mut dim_y, mut dim_z) = (0, 0, 0);
    let mut tets: Vec<_> = input
        .lines()
        .map(|l| {
            let (_, tet) = parse_tet(l).map_err(|e| anyhow!("Parse error: {:?}", e))?;
            dim_x = dim_x.max(tet.end.0);
            dim_y = dim_y.max(tet.end.1);
            dim_z = dim_z.max(tet.end.2);
            Ok(tet)
        })
        .collect::<Result<Vec<_>>>()?;
    tets.sort_by(|a, b| a.start.2.cmp(&b.start.2));
    let mut height_map = Grid::<u16>::new(0, dim_x as usize + 1, dim_y as usize + 1);
    for tet in &mut tets {
        // "Place" tet onto grid
        //      update it's z axis
        //      update height maps

        if tet.start.0 == tet.end.0 && tet.start.1 == tet.end.1 {
            // Vertical tet
            let height = tet.end.2 - tet.start.2 + 1;
            let stack_height = height_map.at_mut(tet.start.0 as usize, tet.start.1 as usize);

            if tet.start.2 < *stack_height {
                return Err(anyhow!("Shouldn't have to lift a tet! {:?}", tet));
            }
            tet.start.2 = *stack_height + 1;
            *stack_height += height;
            tet.end.2 = *stack_height;
            continue;
        }

        if tet.start.0 == tet.end.0 && tet.start.2 == tet.end.2 {
            // Tet along y axis
            let mut new_start_z = 0;
            for i in tet.start.1..=tet.end.1 {
                let stack_height = height_map.at_mut(tet.start.0 as usize, i as usize);
                if tet.start.2 < *stack_height {
                    return Err(anyhow!("Shouldn't have to lift a tet! {:?}", tet));
                }
                new_start_z = new_start_z.max(*stack_height);
            }
            tet.start.2 = new_start_z + 1;
            tet.end.2 = tet.start.2;
            for i in tet.start.1..=tet.end.1 {
                *height_map.at_mut(tet.start.0 as usize, i as usize) = tet.end.2;
            }
            continue;
        }

        if tet.start.1 == tet.end.1 && tet.start.2 == tet.end.2 {
            // Tet along x axis
            let mut new_start_z = 0;
            for i in tet.start.0..=tet.end.0 {
                let stack_height = height_map.at_mut(i as usize, tet.start.1 as usize);
                if tet.start.2 < *stack_height {
                    return Err(anyhow!("Shouldn't have to lift a tet! {:?}", tet));
                }
                new_start_z = new_start_z.max(*stack_height);
            }
            tet.start.2 = new_start_z + 1;
            tet.end.2 = tet.start.2;
            for i in tet.start.0..=tet.end.0 {
                *height_map.at_mut(i as usize, tet.start.1 as usize) = tet.end.2;
            }

            continue;
        }

        return Err(anyhow!("This Tet is wrong! {:?}", tet));
    }

    // Now build a data structure of which tets are supporting which other tets
    // n^2 is fine to start with
    let mut supporters: Vec<Vec<usize>> = vec![vec![]; tets.len()];
    tets.iter().enumerate().for_each(|(i, tet)| {
        tets.iter().take(i).enumerate().for_each(|(j, other_tet)| {
            if other_tet.is_supporting(tet) {
                supporters[i].push(j);
            }
        });
    });

    let mut removable = 0;

    'outer: for (i, _) in tets.iter().enumerate() {
        for s in &supporters {
            if s.len() == 1 && s[0] == i {
                continue 'outer;
            }
        }

        removable += 1;
    }
    let p1 = removable;

    let mut p2_sum = 0;
    // For each tet we find the number of other bricks it transitively supports
    for (i, _) in tets.iter().enumerate() {
        let mut to_remove: BTreeSet<u16> = BTreeSet::new();
        to_remove.insert(i as u16);

        for (j, s_j) in supporters.iter().enumerate() {
            if s_j.len() == 1 && s_j[0] == i {
                to_remove.insert(j as u16);
                continue;
            }
            if !s_j.is_empty() && s_j.iter().all(|x| to_remove.contains(&(*x as u16))) {
                to_remove.insert(j as u16);
                continue;
            }
        }
        p2_sum += to_remove.len() as u64 - 1;
    }

    Ok((p1, p2_sum))
}

#[derive(Debug)]
struct Tetromino {
    start: (u16, u16, u16),
    end: (u16, u16, u16),
}

impl Tetromino {
    fn new(start: (u16, u16, u16), end: (u16, u16, u16)) -> Self {
        Self { start, end }
    }

    fn is_supporting(&self, other: &Self) -> bool {
        if self.start.2 >= other.start.2 {
            return false;
        }

        if self.start.0 == self.end.0 && self.start.1 == self.end.1 {
            // Vertical tet
            return other.has_block(self.start.0, self.start.1, self.end.2 + 1);
        }

        if self.start.0 == self.end.0 && self.start.2 == self.end.2 {
            // Tet along y axis
            for i in self.start.1..=self.end.1 {
                if other.has_block(self.start.0, i, self.end.2 + 1) {
                    return true;
                }
            }
            return false;
        }

        if self.start.1 == self.end.1 && self.start.2 == self.end.2 {
            // Tet along x axis
            for i in self.start.0..=self.end.0 {
                if other.has_block(i, self.start.1, self.end.2 + 1) {
                    return true;
                }
            }
            return false;
        }

        false
    }

    fn has_block(&self, x: u16, y: u16, z: u16) -> bool {
        x >= self.start.0
            && x <= self.end.0
            && y >= self.start.1
            && y <= self.end.1
            && z >= self.start.2
            && z <= self.end.2
    }
}

// Parse lines like 6,0,119~7,0,119
// as two tuples of u16s
fn parse_tet(line: &str) -> IResult<&str, Tetromino> {
    let cm = |s| tag(",")(s);
    let (line, (a, _, b, _, c, _, d, _, e, _, f)) =
        tuple((nu16, cm, nu16, cm, nu16, tag("~"), nu16, cm, nu16, cm, nu16))(line)?;
    let start = (a, b, c);
    let end = (d, e, f);

    debug_assert!(a <= d);
    debug_assert!(b <= e);
    debug_assert!(c <= f);
    // debug_assert!((a == d && b == e) || (a == d && c == f) || (b == e && c == f));

    Ok((line, Tetromino::new(start, end)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part1, 22, Some(5), None);
    sample_test!(sample_part2, 22, None, Some(7));
    input_test!(part1, 22, Some(499), None);
    input_test!(part2, 22, None, Some(95059));
}
