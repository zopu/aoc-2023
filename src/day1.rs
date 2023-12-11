use rayon::prelude::*;

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn run(input: &str) -> color_eyre::Result<(u64, u64)> {
    let (p1, p2) = rayon::join(|| part1(input), || part2(input));
    Ok((p1? as u64, p2? as u64))
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
    use crate::runner::{test_input, test_sample};

    use super::*;

    test_sample!(sample_part1, 1, Some(142), None);
    test_input!(real, 1, Some(55816), Some(54980));

    #[test]
    fn sample_part2() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/1/sample_part2.txt")?;
        assert_eq!(281, part2(&input)?);
        Ok(())
    }
}
