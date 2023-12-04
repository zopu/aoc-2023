use std::collections::HashSet;

use itertools::Itertools;

pub fn run(input: &str) -> color_eyre::Result<(u32, u32)> {
    Ok((part1(input)?, 0))
}

fn part1(input: &str) -> color_eyre::Result<u32> {
    let sum: usize = input
        .lines()
        .map(|line| {
            let (l, r) = line
                .split(':')
                .nth(1)
                .unwrap()
                .split('|')
                .map(|side| {
                    side.split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|n| str::parse::<u32>(n).unwrap())
                })
                .next_tuple()
                .unwrap();
            let winners: HashSet<u32> = HashSet::from_iter(l);
            let nums = HashSet::from_iter(r);
            let mut ans = winners.intersection(&nums).count();
            if ans > 0 {
                ans = 1 << (ans - 1);
            }
            ans
        })
        .sum();
    Ok(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/4/sample.txt")?;
        assert_eq!(13, part1(&input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/4/input.txt")?;
        assert_eq!(17782, part1(&input)?);
        Ok(())
    }
}
