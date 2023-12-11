use color_eyre::Result;

type Point = (usize, usize);

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Point>,
    dimensions: (usize, usize),
}

impl Universe {
    fn expand(&self) -> Self {
        let (mut rows, mut cols) = (vec![0; self.dimensions.1], vec![0; self.dimensions.0]);
        for (c, r) in &self.galaxies {
            rows[*r] = 1;
            cols[*c] = 1;
        }
        let mut expanded = 0;
        for (i, r) in rows.iter_mut().enumerate() {
            if *r == 0 {
                expanded += 1;
            }
            *r = i + expanded;
        }
        let d_rows = expanded;
        let mut expanded = 0;
        for (i, c) in cols.iter_mut().enumerate() {
            if *c == 0 {
                expanded += 1;
            }
            *c = i + expanded;
        }
        let d_cols = expanded;
        let galaxies = self
            .galaxies
            .iter()
            .map(|(c, r)| (cols[*c], rows[*r]))
            .collect();

        Universe {
            galaxies,
            dimensions: (self.dimensions.0 + d_cols, self.dimensions.1 + d_rows),
        }
    }
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let universe = parse(input)?;
    let universe = universe.expand();
    let mut sum: u64 = 0;
    for g1 in &universe.galaxies {
        for g2 in &universe.galaxies {
            sum += manhattan(g1, g2) as u64;
        }
    }
    Ok((sum / 2, 0))
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
    use crate::runner::{test_input, test_sample};

    test_sample!(sample_part1, 11, Some(374), None);
    test_input!(part1, 11, Some(9329143), None);
}
