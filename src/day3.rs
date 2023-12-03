pub fn run(input: &str) -> color_eyre::Result<(u32, u32)> {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> color_eyre::Result<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let parts = find_parts(&grid);
    Ok(parts.iter().map(|(n, _, _, _)| n).sum())
}

fn part2(input: &str) -> color_eyre::Result<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let parts = find_parts(&grid);

    let mut sum = 0;
    for (line_num, line) in grid.iter().enumerate() {
        for (i, c) in line.iter().enumerate() {
            if *c == '*' {
                let adjacent_parts: Vec<_> = parts
                    .iter()
                    .filter(|(_, l, st, end)| is_adjacent(line_num, i, *l, *st, *end))
                    .collect();
                if adjacent_parts.len() == 2 {
                    sum += adjacent_parts[0].0 * adjacent_parts[1].0;
                }
            }
        }
    }

    Ok(sum)
}

pub fn find_parts(grid: &[Vec<char>]) -> Vec<(u32, usize, usize, usize)> {
    let mut v: Vec<(u32, usize, usize, usize)> = Vec::new();
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

        let it = nums
            .iter()
            .filter(|(_, start, end)| has_adjacent_special_chars(grid, line_num, *start, *end))
            .map(|(n, start, end)| (*n, line_num, *start, *end));
        v.extend(it);
    }
    v
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
    !(c.is_ascii_digit() || c == '.')
}

fn is_adjacent(
    line: usize,
    col: usize,
    part_line: usize,
    part_start: usize,
    part_end: usize,
) -> bool {
    (line as i32 - part_line as i32).abs() <= 1
        && col as i32 >= part_start as i32 - 1
        && col < part_end + 1
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
    fn test_part2() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/3/input.txt")?;
        assert_eq!(73074886, part2(&input)?);
        Ok(())
    }

    #[test]
    fn test_sample_part2() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/3/sample.txt")?;
        assert_eq!(467835, part2(&input)?);
        Ok(())
    }

    #[test]
    fn test_diagonal_input() -> color_eyre::Result<()> {
        let input = "*...\n.123";
        assert_eq!(123, part1(&input)?);
        Ok(())
    }
}
