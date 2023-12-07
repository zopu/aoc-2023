use std::cmp::Ordering;

use color_eyre::Result;
use rayon::{iter::ParallelIterator, str::ParallelString};

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    typ: HandType,
    typ_with_wild_jokers: HandType,
}

impl Hand {
    fn compare(&self, other: &Self, jokers_wild: bool) -> Ordering {
        if jokers_wild {
            match self.typ_with_wild_jokers.cmp(&other.typ_with_wild_jokers) {
                Ordering::Equal => {}
                ord => return ord,
            }
        } else {
            match self.typ.cmp(&other.typ) {
                Ordering::Equal => {}
                ord => return ord,
            }
        }

        // Compare cards
        for (a, b) in self.cards.into_iter().zip(other.cards.into_iter()) {
            match (a, b, jokers_wild) {
                (11, 11, _) => {}
                (11, _, true) => return Ordering::Less,
                (_, 11, true) => return Ordering::Greater,
                (a, b, _) if a > b => return Ordering::Greater,
                (a, b, _) if a < b => return Ordering::Less,
                _ => {}
            }
        }
        Ordering::Equal
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
    fn from((cards, jokers_wild): ([u8; 5], bool)) -> Self {
        let mut card_counts = [0u8; 15];
        for c in cards {
            card_counts[c as usize] += 1;
        }
        let num_jokers = if jokers_wild { card_counts[11] } else { 0 };
        if jokers_wild {
            card_counts[11] = 0
        };
        let mut counts: Vec<_> = card_counts.iter().filter(|&&c| c > 0).collect();

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
    let mut v: Vec<(Hand, Bid)> = input
        .par_lines()
        .map(parse_hand)
        .collect::<Result<Vec<_>>>()?;
    Ok((solve(&mut v, false)?, solve(&mut v, true)?))
}

fn solve(v: &mut [(Hand, Bid)], jokers_wild: bool) -> Result<u64> {
    v.sort_by(|(a, _), (b, _)| a.compare(b, jokers_wild));
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
        typ: HandType::from((cards, false)),
        typ_with_wild_jokers: HandType::from((cards, true)),
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
