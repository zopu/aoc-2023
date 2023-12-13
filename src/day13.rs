use color_eyre::Result;
use rayon::iter::{ParallelBridge, ParallelIterator};

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
    let (p1, p2): (u64, u64) = input
        .split("\n\n")
        .par_bridge()
        .map(Grid::from)
        .map(|g| {
            let row_symmetry = find_symmetry(&g.rows);
            let col_symmetry = find_symmetry(&g.cols);
            let oo_row_symmetry = find_one_off_symmetry(&g.rows);
            let oo_col_symmetry = find_one_off_symmetry(&g.cols);
            (
                col_symmetry + 100 * row_symmetry,
                oo_col_symmetry + 100 * oo_row_symmetry,
            )
        })
        .reduce(|| (0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2));
    Ok((p1, p2))
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

fn find_one_off_symmetry(nums: &[u64]) -> u64 {
    'outer: for i in 1..nums.len() {
        let mut bit_diff = 0;
        for j in 1..=i {
            if i + j > nums.len() {
                if bit_diff == 1 {
                    return i as u64;
                } else {
                    continue 'outer;
                }
            }
            bit_diff += (nums[i + j - 1] ^ nums[i - j]).count_ones();
            if bit_diff > 1 {
                continue 'outer;
            }
        }
        if bit_diff == 1 {
            return i as u64;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part1, 13, Some(405), None);
    sample_test!(sample_part2, 13, None, Some(400));
    input_test!(part1, 13, Some(37718), None);
}
