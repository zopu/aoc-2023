pub fn run(input: &str) -> color_eyre::Result<(u32, u32)> {
    Ok((part1(input)?, 0))
}

fn part1(input: &str) -> color_eyre::Result<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut sum: u32 = 0;
    for (line_num, l) in grid.iter().enumerate() {
        let mut nums: Vec<(u32, usize, usize)> = Vec::new();
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
                    nums.push((n, num_start, i));
                    in_num = false;
                }
            }
        }
        if in_num {
            nums.push((n, num_start, l.len()));
        }

        sum += nums
            .iter()
            .filter(|(_, start, end)| has_adjacent_special_chars(&grid, line_num, *start, *end))
            .map(|(n, _, _)| n)
            .sum::<u32>();
    }

    Ok(sum)
}

pub fn has_adjacent_special_chars(
    grid: &[Vec<char>],
    line: usize,
    start: usize,
    end: usize,
) -> bool {
    if start > 0 && is_special(grid[line][start - 1]) {
        return true;
    }
    if end < grid[line].len() - 1 && is_special(grid[line][end]) {
        return true;
    }
    let st = if start > 0 { start - 1 } else { start };
    let ed = if end < grid[0].len() - 1 {
        end + 1
    } else {
        end
    };
    let check_line = |l: usize| grid[l][st..ed].iter().any(|c| is_special(*c));
    if line > 0 && check_line(line - 1) {
        return true;
    }
    if line < grid.len() - 1 && check_line(line + 1) {
        return true;
    }
    false
}

fn is_special(c: char) -> bool {
    if c.is_ascii_digit() {
        return false;
    }
    if c == '.' {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/3/sample.txt")?;
        assert_eq!(4361, part1(&input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/3/input.txt")?;
        assert_eq!(527369, part1(&input)?);
        Ok(())
    }

    #[test]
    fn test_diagonal_input() -> color_eyre::Result<()> {
        let input = "*...\n.123";
        assert_eq!(123, part1(&input)?);
        Ok(())
    }
}
