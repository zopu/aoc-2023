use rayon::prelude::*;

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn run(input: &str) -> color_eyre::Result<(u32, u32)> {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> color_eyre::Result<u32> {
    let sum = input
        .par_lines()
        .map(|l| {
            let digits = l.chars().filter_map(|c| c.to_digit(10));
            let first = digits.clone().next().unwrap();
            let last = digits.last().unwrap();

            first * 10 + last
        })
        .sum();
    Ok(sum)
}

fn part2(input: &str) -> color_eyre::Result<u32> {
    let sum = input
        .par_lines()
        .map(|l| {
            let mut digits = Vec::<u32>::new();
            let mut it = l.chars();
            loop {
                if let Some(d) = leading_digit(it.as_str()) {
                    digits.push(d);
                }
                let next = it.next();
                if next.is_none() {
                    break;
                }
            }
            digits[0] * 10 + digits[digits.len() - 1]
        })
        .sum();
    Ok(sum)
}

fn leading_digit(s: &str) -> Option<u32> {
    if s.is_empty() {
        return None;
    };
    let first = s.chars().next()?;
    if first.is_ascii_digit() {
        return Some(u32::from(first) - '0' as u32);
    };
    for (i, d) in DIGITS.iter().enumerate().skip(1) {
        if s.starts_with(d) {
            return Some(i as u32);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/1/sample.txt")?;
        assert_eq!(142, part1(&input)?);
        Ok(())
    }

    #[test]
    fn test_sample_part2() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/1/sample_part2.txt")?;
        assert_eq!(281, part2(&input)?);
        Ok(())
    }

    #[test]
    fn test_real() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/1/input.txt")?;
        let (p1, p2) = run(&input)?;
        assert_eq!(55816, p1);
        assert_eq!(54980, p2);
        Ok(())
    }
}
