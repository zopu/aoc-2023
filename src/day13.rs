use color_eyre::Result;
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Debug)]
struct Grid {
    dimensions: (usize, usize),
    rows: [u32; 20],
    cols: [u32; 20],
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let (mut rows, mut cols) = ([0; 20], [0; 20]);
        let mut row_count = 0;
        let mut col_count = 0;
        for (l_i, line) in input.lines().enumerate() {
            let mut row = 0;
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    row |= 1 << i;
                    cols[i] |= 1 << l_i;
                }
                col_count += 1;
            }
            rows[l_i] = row;
            row_count += 1;
        }
        let dimensions = (row_count, col_count / row_count);
        Self {
            dimensions,
            rows,
            cols,
        }
    }
}

impl Grid {
    fn rows(&self) -> &[u32] {
        self.rows[0..self.dimensions.0].as_ref()
    }
    fn cols(&self) -> &[u32] {
        self.cols[0..self.dimensions.1].as_ref()
    }
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let (p1, p2): (u32, u32) = input
        .split("\n\n")
        .par_bridge()
        .map(Grid::from)
        .map(|g| {
            // println!("{:?}", g);
            let row_symmetry = find_symmetry(g.rows());
            let col_symmetry = find_symmetry(g.cols());
            let oo_row_symmetry = find_one_off_symmetry(g.rows());
            let oo_col_symmetry = find_one_off_symmetry(g.cols());
            (
                col_symmetry + 100 * row_symmetry,
                oo_col_symmetry + 100 * oo_row_symmetry,
            )
        })
        .reduce(|| (0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2));
    Ok((p1 as u64, p2 as u64))
}

fn find_symmetry(nums: &[u32]) -> u32 {
    'outer: for i in 1..nums.len() {
        // Check if there is a symmetry line before row i
        for j in 0..=i {
            if i + j > nums.len() {
                return i as u32;
            }
            if nums[i + j - 1] != nums[i - j] {
                continue 'outer;
            }
        }
        return i as u32;
    }
    0
}

fn find_one_off_symmetry(nums: &[u32]) -> u32 {
    'outer: for i in 1..nums.len() {
        let mut bit_diff = 0;
        for j in 1..=i {
            if i + j > nums.len() {
                if bit_diff == 1 {
                    return i as u32;
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
            return i as u32;
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
