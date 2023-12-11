use std::collections::HashSet;

use color_eyre::eyre::anyhow;
use color_eyre::Result;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::{bytes::complete::tag, sequence::tuple};
use nom::{
    character::complete::{multispace0, multispace1, u32},
    error::Error,
};
use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn run(input: &str) -> Result<(u64, u64)> {
    let matches = input
        .par_lines()
        .map(scratchcard_matches)
        .collect::<Result<Vec<_>, _>>()?;

    Ok((part1(&matches)? as u64, part2(&matches)? as u64))
}

fn part1(matches: &[usize]) -> Result<u32> {
    Ok(matches
        .iter()
        .map(|m| if *m > 0 { 1 << (m - 1) } else { 0 })
        .sum())
}

fn part2(matches: &[usize]) -> Result<u32> {
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

fn parse_card(line: &str) -> Result<(Vec<u32>, Vec<u32>)> {
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

fn scratchcard_matches(card: &str) -> Result<usize> {
    let (l, r) = parse_card(card)?;
    let winners: HashSet<u32> = HashSet::from_iter(l);
    Ok(r.iter().filter(|n| winners.contains(n)).count())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part1, 4, Some(13), None);
    sample_test!(sample_part2, 4, None, Some(30));
    input_test!(part1, 4, Some(17782), None);
    input_test!(part2, 4, None, Some(8477787));

    #[test]
    fn test_parse_line() {
        let line = "Card   3:  4 45 78 42 29 92 16 90 93 30 | 97 90 75 40 43 65 92 83 41  4 47 35 29 80 68 87 30 71 98 42 95  7 76 69 88";
        let (l, r) = parse_card(line).unwrap();
        assert_eq!(10, l.len());
        assert_eq!(25, r.len());
        assert_eq!(90, r[1]);
    }
}
