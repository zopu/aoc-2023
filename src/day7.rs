use std::cmp::Ordering;

use color_eyre::Result;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    typ: HandType,
    jokers_wild: bool,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.typ.cmp(&other.typ) {
            Ordering::Equal => {}
            ord => return ord,
        }
        // Compare cards
        for (a, b) in self.cards.iter().zip(other.cards.iter()) {
            if a > b {
                return Ordering::Greater;
            }
            if b > a {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Bid = u32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<([u8; 5], bool)> for HandType {
    fn from(input: ([u8; 5], bool)) -> Self {
        let (cards, _jokers_wild) = input;
        let num_jokers = cards.iter().filter(|n| **n == 0).count();
        let hashmap = cards.iter().filter(|n| **n != 0).counts();
        let mut counts: Vec<_> = hashmap.values().collect();
        if counts.len() < 2 {
            return HandType::FiveOfAKind;
        }
        counts.sort();
        let (max, next) = (counts[counts.len() - 1], counts[counts.len() - 2]);
        match (max + num_jokers, next) {
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, 1) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> Result<u64> {
    let mut v: Vec<(Hand, Bid)> = input
        .lines()
        .map(|l| parse_hand(l, false))
        .collect::<Result<Vec<_>>>()?;
    v.sort();
    let sum: u64 = v
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| *bid as u64 * (i as u64 + 1))
        .sum();
    Ok(sum)
}

fn part2(input: &str) -> Result<u64> {
    let mut v: Vec<(Hand, Bid)> = input
        .lines()
        .map(|l| parse_hand(l, true))
        .collect::<Result<Vec<_>>>()?;
    v.sort();
    let sum: u64 = v
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| *bid as u64 * (i as u64 + 1))
        .sum();
    Ok(sum)
}

fn parse_hand(line: &str, jokers_wild: bool) -> Result<(Hand, Bid)> {
    let mut cards = [0; 5];
    for (i, c) in line.chars().take(5).enumerate() {
        let n = match (c, c.is_ascii_digit()) {
            (_, true) => c as u8 - b'0',
            ('T', _) => 10,
            ('J', _) => {
                if jokers_wild {
                    0
                } else {
                    11
                }
            }
            ('Q', _) => 12,
            ('K', _) => 13,
            ('A', _) => 14,
            _ => 0,
        };
        cards[i] = n;
    }
    let bid = line.chars().skip(6).collect::<String>().parse()?;
    let hand = Hand {
        cards,
        typ: HandType::from((cards, jokers_wild)),
        jokers_wild,
    };
    Ok((hand, bid))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::{test_input, test_sample};

    test_sample!(sample_part1, 7, Some(6440), None);
    test_sample!(sample_part2, 7, None, Some(5905));
    test_input!(part1, 7, Some(250347426), None);
    test_input!(part2, 7, None, Some(251224870));
}
