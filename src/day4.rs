use std::collections::HashSet;

use color_eyre::eyre::anyhow;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::{bytes::complete::tag, sequence::tuple};
use nom::{
    character::complete::{multispace0, multispace1, u32},
    error::Error,
};
use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn run(input: &str) -> color_eyre::Result<(u32, u32)> {
    let matches: Vec<_> = input
        .par_lines()
        .map(|line| scratchcard_matches(line).unwrap())
        .collect();
    Ok((part1(&matches)?, part2(&matches)?))
}

fn part1(matches: &[usize]) -> color_eyre::Result<u32> {
    Ok(matches
        .iter()
        .map(|m| if *m > 0 { 1 << (m - 1) } else { 0 })
        .sum())
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

fn parse_card(line: &str) -> color_eyre::Result<(Vec<u32>, Vec<u32>)> {
    let (_remaining, (l, r)) = preceded(
        tuple((tag("Card"), multispace0, u32, tag(":"), multispace0)),
        separated_pair(
            separated_list1(multispace1, u32),
            tuple((multispace0, tag("|"), multispace0)),
            separated_list1(multispace1, u32),
        ),
    )(line)
    .map_err(|e: nom::Err<Error<_>>| anyhow!("Parse error: {}", e))?;
    Ok((l, r))
}

fn scratchcard_matches(card: &str) -> color_eyre::Result<usize> {
    let (l, r) = parse_card(card)?;
    let winners: HashSet<u32> = HashSet::from_iter(l);
    Ok(r.iter().filter(|n| winners.contains(n)).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() -> color_eyre::Result<()> {
        let line = "Card   3:  4 45 78 42 29 92 16 90 93 30 | 97 90 75 40 43 65 92 83 41  4 47 35 29 80 68 87 30 71 98 42 95  7 76 69 88";
        let (l, r) = parse_card(line)?;
        assert_eq!(10, l.len());
        assert_eq!(25, r.len());
        assert_eq!(90, r[1]);
        Ok(())
    }

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
