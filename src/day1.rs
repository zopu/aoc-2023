const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn day1(input: &str) -> color_eyre::Result<(u32, u32)> {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> color_eyre::Result<u32> {
    let sum = input.lines().fold(0, |sum, l| {
        let mut digits = l.chars().filter(|c| c.is_ascii_digit());
        let first = digits.next();
        let last = digits.last().or(first);

        let mut s = first.unwrap().to_string();
        s.push(last.unwrap());

        let num: u32 = s.parse().unwrap();
        sum + num
    });
    Ok(sum)
}

fn part2(input: &str) -> color_eyre::Result<u32> {
    let sum = input.lines().fold(0, |sum, l| {
        let mut s = l;
        let mut digits = Vec::<u32>::new();
        while !s.is_empty() {
            if let Some(d) = leading_digit(s) {
                digits.push(d);
            }
            s = &s[1..];
        }
        let n = digits[0] * 10 + digits[digits.len() - 1];
        sum + n
    });
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
    fn test_sample_part1() {
        let input = std::fs::read_to_string("inputs/1/sample.txt").unwrap();
        assert_eq!(142, part1(&input).unwrap());
    }

    #[test]
    fn test_sample_part2() {
        let input = std::fs::read_to_string("inputs/1/sample_part2.txt").unwrap();
        assert_eq!(281, part2(&input).unwrap());
    }

    #[test]
    fn test_real() {
        let input = std::fs::read_to_string("inputs/1/input.txt").unwrap();
        let (p1, p2) = day1(&input).unwrap();
        assert_eq!(55816, p1);
        assert_eq!(54980, p2);
    }
}
