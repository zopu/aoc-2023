use color_eyre::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u8,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Spring {
    Good,
    Bad,
    Unknown,
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let parsed: Vec<(_, _)> = input
        .lines()
        .map(|l| parse_line(l).unwrap())
        .map(|(_, (s, g))| (s, g))
        .collect();
    let p1 = parsed
        .iter()
        .map(|(s, g)| count_combinations(s, g, &mut Cache::new()))
        .sum();

    let p2_parsed: Vec<_> = parsed
        .iter()
        .map(|(s, g)| {
            let mut s5 = s.clone();
            s5.push(Spring::Unknown);
            s5.extend(s.iter());
            s5.push(Spring::Unknown);
            s5.extend(s.iter());
            s5.push(Spring::Unknown);
            s5.extend(s.iter());
            s5.push(Spring::Unknown);
            s5.extend(s.iter());
            let g5 = g.repeat(5);
            (s5, g5)
        })
        .collect();
    let p2: u64 = p2_parsed
        .par_iter()
        .map(|(s, g)| count_combinations(s, g, &mut Cache::new()))
        .sum();
    Ok((p1, p2))
}

#[derive(Hash, PartialEq, Eq)]
struct CacheKey {
    key: u16,
}

impl<'a> From<(&'a [Spring], &'a [u8])> for CacheKey {
    fn from((springs, groups): (&'a [Spring], &'a [u8])) -> Self {
        CacheKey {
            key: u16::from_be_bytes([springs.len() as u8, groups.len() as u8]),
        }
    }
}

struct Cache {
    cache: FxHashMap<u16, u64>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            cache: FxHashMap::default(),
        }
    }

    fn get(&self, key: &CacheKey) -> Option<u64> {
        self.cache.get(&(key.key)).copied()
    }

    fn set(&mut self, key: CacheKey, value: u64) {
        self.cache.insert(key.key, value);
    }
}

fn count_combinations<'a>(springs: &'a [Spring], groups: &'a [u8], cache: &mut Cache) -> u64 {
    let key = (springs, groups).into();
    if let Some(result) = cache.get(&key) {
        return result;
    };

    // println!("Checking: {:?}, {:?}", springs, groups);
    if springs.is_empty() && groups.is_empty() {
        cache.set(key, 1);
        return 1;
    }
    if springs.is_empty() {
        cache.set(key, 0);
        return 0;
    }
    if groups.is_empty() {
        // Check that no remaining springs are bad
        if springs.iter().any(|s| matches!(s, Spring::Bad)) {
            cache.set(key, 0);
            return 0;
        } else {
            cache.set(key, 1);
            return 1;
        }
    };
    if groups[0] as usize > springs.len() {
        cache.set(key, 0);
        return 0;
    }

    // If the first group matches, then consume and check the rest
    let first_group_matches = springs[0..(groups[0] as usize)]
        .iter()
        .all(|s| *s != Spring::Good);
    let trailing_not_bad =
        (groups[0] as usize == springs.len()) || springs[groups[0] as usize] != Spring::Bad;
    if first_group_matches && trailing_not_bad {
        let a = if groups[0] as usize == springs.len() {
            if groups.len() == 1 {
                1
            } else {
                0
            }
        } else {
            {
                count_combinations(&springs[(groups[0] as usize + 1)..], &groups[1..], cache)
            }
        };
        let b = if springs[0] == Spring::Bad {
            0
        } else {
            count_combinations(&springs[1..], groups, cache)
        };
        cache.set(key, a + b);
        return a + b;
    }

    if springs[0] == Spring::Bad {
        cache.set(key, 0);
        return 0;
    }

    // Else just check the rest
    let result = count_combinations(&springs[1..], groups, cache);
    cache.set(key, result);
    result
}

// Parse lines like: #.#.### 1,1,3
fn parse_line(input: &str) -> IResult<&str, (Vec<Spring>, Vec<u8>)> {
    let (remaining, (springs, groups)) = separated_pair(
        many1(alt((tag("#"), tag("."), tag("?")))),
        tag(" "),
        separated_list1(tag(","), u8),
    )(input)?;
    let springs = springs
        .into_iter()
        .map(|s| match s {
            "#" => Spring::Bad,
            "." => Spring::Good,
            "?" => Spring::Unknown,
            _ => unreachable!(),
        })
        .collect();
    Ok((remaining, (springs, groups)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part1, 12, Some(21), None);
    sample_test!(sample_part2, 12, None, Some(525152));
    input_test!(part1, 12, Some(6852), None);
    input_test!(part2, 12, None, Some(8475948826693));

    #[test]
    fn test_weird_case() {
        let springs = vec![Spring::Bad];
        let groups = vec![1, 3];
        assert_eq!(0, count_combinations(&springs, &groups, &mut Cache::new()));
    }

    #[test]
    fn last_sample_case() {
        let line = "?###???????? 3,2,1";
        let (_remaining, (springs, groups)) = parse_line(line).unwrap();
        assert_eq!(10, count_combinations(&springs, &groups, &mut Cache::new()));
    }

    #[test]
    fn empty_springs() {
        let springs = vec![];
        let groups = vec![];
        assert_eq!(1, count_combinations(&springs, &groups, &mut Cache::new()));
    }

    #[test]
    fn empty_groups() {
        let springs = vec![Spring::Bad];
        let groups = vec![];
        assert_eq!(0, count_combinations(&springs, &groups, &mut Cache::new()));
        let springs = vec![Spring::Good];
        let groups = vec![];
        assert_eq!(1, count_combinations(&springs, &groups, &mut Cache::new()));
    }

    #[test]
    fn extra_bad() {
        let springs = vec![Spring::Bad, Spring::Bad];
        let groups = vec![1];
        assert_eq!(0, count_combinations(&springs, &groups, &mut Cache::new()));
    }

    #[test]
    fn long_case() {
        let line = ".???.???...?????? 1,1,5";
        let (_remaining, (springs, groups)) = parse_line(line).unwrap();
        assert_eq!(22, count_combinations(&springs, &groups, &mut Cache::new()));
    }

    #[test]
    fn very_high_case() {
        let line = "??????.???. 1,1,1";
        let (_remaining, (springs, groups)) = parse_line(line).unwrap();
        assert_eq!(40, count_combinations(&springs, &groups, &mut Cache::new()));
    }
}
