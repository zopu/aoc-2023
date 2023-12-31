use color_eyre::Result;

type Point = (usize, usize);

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Point>,
    dimensions: (usize, usize),
}

impl Universe {
    fn expand(&self, expansion: usize) -> Self {
        let (mut rows, mut cols) = (vec![0; self.dimensions.1], vec![0; self.dimensions.0]);
        for &(c, r) in &self.galaxies {
            (rows[r], cols[c]) = (1, 1);
        }
        let d_rows = rows.iter_mut().enumerate().fold(0, |mut expanded, (i, r)| {
            if *r == 0 {
                expanded += expansion - 1;
            }
            *r = i + expanded;
            expanded
        });
        let d_cols = cols.iter_mut().enumerate().fold(0, |mut expanded, (i, c)| {
            if *c == 0 {
                expanded += expansion - 1;
            }
            *c = i + expanded;
            expanded
        });
        let galaxies = self
            .galaxies
            .iter()
            .map(|&(c, r)| (cols[c], rows[r]))
            .collect();

        Universe {
            galaxies,
            dimensions: (self.dimensions.0 + d_cols, self.dimensions.1 + d_rows),
        }
    }
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let universe = parse(input)?;
    let (p1, p2) = rayon::join(|| solve(&universe, 2), || solve(&universe, 1_000_000));
    Ok((p1, p2))
}

fn solve(universe: &Universe, expansion: usize) -> u64 {
    let universe = universe.expand(expansion);
    let mut sum = 0;
    for (i, g1) in universe.galaxies.iter().enumerate() {
        for g2 in &universe.galaxies[i..] {
            sum += manhattan(g1, g2) as u64;
        }
    }
    sum
}

fn manhattan(p1: &Point, p2: &Point) -> u32 {
    (p1.0 as i32 - p2.0 as i32).unsigned_abs() + (p1.1 as i32 - p2.1 as i32).unsigned_abs()
}

fn parse(input: &str) -> Result<Universe> {
    let mut galaxies: Vec<Point> = Vec::new();
    let dimensions = (input.lines().next().unwrap().len(), input.lines().count());
    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                galaxies.push((j, i));
            }
        })
    });
    Ok(Universe {
        galaxies,
        dimensions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part1, 11, Some(374), None);
    sample_test!(sample_part2, 11, None, Some(82000210));
    input_test!(part1, 11, Some(9329143), None);
    input_test!(part2, 11, None, Some(710674907809));

    #[test]
    fn sample_part2_at_different_scales() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/11/sample.txt")?;
        let universe = parse(&input)?;
        assert_eq!(1030, solve(&universe, 10));
        assert_eq!(8410, solve(&universe, 100));
        Ok(())
    }
}
