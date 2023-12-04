use std::collections::HashSet;

use itertools::Itertools;

pub fn run(input: &str) -> color_eyre::Result<(u32, u32)> {
    let matches: Vec<_> = input
        .lines()
        .map(|line| scratchcard_matches(line).unwrap())
        .collect();
    Ok((part1(&matches)?, part2(&matches)?))
}

fn part1(matches: &[usize]) -> color_eyre::Result<u32> {
    let sum: usize = matches
        .iter()
        .map(|m| if *m > 0 { 1 << (m - 1) } else { 0 })
        .sum();
    Ok(sum as u32)
}

fn part2(matches: &[usize]) -> color_eyre::Result<u32> {
    let mut cards = vec![1; matches.len()];
    for i in 0..cards.len() {
        for j in 1..(matches[i] + 1) {
            if i + j < cards.len() {
                cards[i + j] += cards[i];
            }
        }
    }
    Ok(cards.iter().sum())
}

fn scratchcard_matches(card: &str) -> color_eyre::Result<usize> {
    let (l, r) = card
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
    Ok(winners.intersection(&nums).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/4/sample.txt")?;
        assert_eq!(13, run(&input)?.0);
        Ok(())
    }

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/4/input.txt")?;
        assert_eq!(17782, run(&input)?.0);
        Ok(())
    }

    #[test]
    fn test_sample_part2() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/4/sample.txt")?;
        assert_eq!(30, run(&input)?.1);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/4/input.txt")?;
        assert_eq!(8477787, run(&input)?.1);
        Ok(())
    }
}
