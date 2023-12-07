use std::cmp::Ordering;

use color_eyre::Result;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    typ: HandType,
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

impl From<[u8; 5]> for HandType {
    fn from(cards: [u8; 5]) -> Self {
        let hashmap = cards.iter().counts();
        let mut counts: Vec<_> = hashmap.values().collect();
        if counts.len() == 1 {
            return HandType::FiveOfAKind;
        }
        counts.sort();
        let (max, next) = (counts[counts.len() - 1], counts[counts.len() - 2]);
        match (max, next) {
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
    Ok((part1(input)? as u64, 0))
}

fn part1(input: &str) -> Result<u64> {
    let mut v: Vec<(Hand, Bid)> = input.lines().map(parse_hand).collect::<Result<Vec<_>>>()?;
    v.sort();
    let sum: u64 = v
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| *bid as u64 * (i as u64 + 1))
        .sum();
    Ok(sum)
}

fn parse_hand(line: &str) -> Result<(Hand, Bid)> {
    let mut cards = [0; 5];
    for (i, c) in line.chars().take(5).enumerate() {
        let n = match (c, c.is_ascii_digit()) {
            (_, true) => c as u8 - b'0',
            ('T', _) => 10,
            ('J', _) => 11,
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
        typ: HandType::from(cards),
    };
    Ok((hand, bid))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test_sample;

    test_sample!(sample_part1, 7, Some(6440), None);
}
