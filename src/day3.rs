use std::cmp::min;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Clone)]
struct Part {
    num: u32,
    line: usize,
    start: usize,
    end: usize,
}

pub fn run(input: &str) -> color_eyre::Result<(u64, u64)> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let parts = find_parts(&grid);
    Ok((part1(&parts)? as u64, part2(&grid, &parts)? as u64))
}

fn part1(parts: &[Part]) -> color_eyre::Result<u32> {
    Ok(parts.iter().map(|p| p.num).sum())
}

fn part2(grid: &[Vec<char>], parts: &[Part]) -> color_eyre::Result<u32> {
    let sum = grid
        .par_iter()
        .enumerate()
        .map(|(line_num, line)| {
            let mut line_sum = 0;
            for (i, c) in line.iter().enumerate() {
                if *c == '*' {
                    let mut it = parts
                        .iter()
                        .filter(|p| is_adjacent(line_num, i, p))
                        .enumerate();
                    if let Some((_i, frst)) = it.next() {
                        if let Some((count, lst)) = it.last() {
                            if count == 1 {
                                line_sum += frst.num * lst.num;
                            }
                        }
                    }
                }
            }
            line_sum
        })
        .sum();

    Ok(sum)
}

fn find_parts(grid: &[Vec<char>]) -> Vec<Part> {
    let mut nums: Vec<Part> = Vec::new();
    for (line_num, l) in grid.iter().enumerate() {
        let mut n = 0;
        let mut in_num = false;
        let mut num_start = 0;
        for (i, c) in l.iter().enumerate() {
            match (c.to_digit(10), in_num) {
                (None, false) => {}
                (Some(d), false) => {
                    // Starting the number
                    n = d;
                    num_start = i;
                    in_num = true;
                }
                (Some(d), true) => {
                    // Continuing the number
                    n = n * 10 + d;
                }
                (None, true) => {
                    // End the number
                    nums.push(Part {
                        num: n,
                        line: line_num,
                        start: num_start,
                        end: i,
                    });
                    in_num = false;
                }
            }
        }
        if in_num {
            nums.push(Part {
                num: n,
                line: line_num,
                start: num_start,
                end: l.len(),
            });
        }
    }
    let filtered: Vec<Part> = nums
        .into_iter()
        .filter(|p| has_adjacent_special_chars(grid, p.line, p.start, p.end))
        .collect();
    filtered
}

pub fn has_adjacent_special_chars(
    grid: &[Vec<char>],
    line: usize,
    start: usize,
    end: usize,
) -> bool {
    if start > 0 && is_special(grid[line][start - 1])
        || end < grid[line].len() - 1 && is_special(grid[line][end])
    {
        return true;
    }
    let st = if start > 0 { start - 1 } else { start };
    let ed = min(end + 1, grid[0].len() - 1);
    let check_line = |l: usize| grid[l][st..ed].iter().any(|c| is_special(*c));
    line > 0 && check_line(line - 1) || line < grid.len() - 1 && check_line(line + 1)
}

fn is_special(c: char) -> bool {
    !(c.is_ascii_digit() || c == '.')
}

fn is_adjacent(line: usize, col: usize, part: &Part) -> bool {
    (line as i32 - part.line as i32).abs() <= 1
        && col as i32 >= part.start as i32 - 1
        && col < part.end + 1
}

#[cfg(test)]
mod tests {
    use crate::runner::{test_input, test_sample};

    use super::*;

    test_sample!(sample_part1, 3, Some(4361), None);
    test_input!(part1, 3, Some(527369), None);
    test_input!(part2, 3, None, Some(73074886));
    test_sample!(sample_part2, 3, None, Some(467835));

    #[test]
    fn diagonal_input() {
        let input = "*...\n.123";
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let parts = find_parts(&grid);
        super::part1(&parts).unwrap();
    }
}
