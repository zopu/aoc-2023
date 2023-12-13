use color_eyre::Result;

#[derive(Debug)]
struct Grid {
    rows: Vec<u64>,
    cols: Vec<u64>,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut rows = Vec::new();
        let mut cols = vec![0; input.lines().next().unwrap().len()];
        for (l_i, line) in input.lines().enumerate() {
            let mut row = 0;
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    row |= 1 << i;
                    cols[i] |= 1 << l_i;
                }
            }
            rows.push(row);
        }
        Self { rows, cols }
    }
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let grids = parse(input)?;
    let p1: u64 = grids
        .iter()
        .map(|g| {
            let row_symmetry = find_symmetry(&g.rows);
            let col_symmetry = find_symmetry(&g.cols);
            col_symmetry + 100 * row_symmetry
        })
        .sum();
    Ok((p1, 0))
}

fn find_symmetry(nums: &[u64]) -> u64 {
    'outer: for i in 1..nums.len() {
        // Check if there is a symmetry line before row i
        for j in 0..=i {
            if i + j > nums.len() {
                return i as u64;
            }
            if nums[i + j - 1] != nums[i - j] {
                continue 'outer;
            }
        }
        return i as u64;
    }
    0
}

fn parse(input: &str) -> Result<Vec<Grid>> {
    let mut grids: Vec<String> = vec![];
    grids.push(String::from(""));
    input.lines().for_each(|l| {
        if l.is_empty() {
            grids.push(String::from(""));
        } else {
            let grid = grids.last_mut().unwrap();
            grid.push_str(l);
            grid.push('\n');
        }
    });

    Ok(grids.iter().map(|s| Grid::from(s.as_str())).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part1, 13, Some(405), None);
    input_test!(part1, 13, Some(37718), None);
}
