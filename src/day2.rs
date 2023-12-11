use rayon::prelude::*;
use std::cmp::max;

#[derive(Clone, Debug)]
pub struct Set {
    r: u32,
    g: u32,
    b: u32,
}

mod parse {
    use super::Set;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::u32;
    use nom::error::{Error, ErrorKind};
    use nom::multi::separated_list0;
    use nom::sequence::tuple;
    use nom::{combinator::value, sequence::separated_pair, IResult};

    pub fn parse_game(input: &str) -> IResult<&str, (u32, Vec<Set>)> {
        let (remaining, (_, num, _)) = tuple((tag("Game "), u32, tag(": ")))(input)?;
        let (remaining, game_result) = separated_list0(tag("; "), parse_set)(remaining)?;
        Ok((remaining, (num, game_result)))
    }

    pub fn parse_set(input: &str) -> IResult<&str, Set> {
        let (remaining, color_vals) = separated_list0(
            tag(", "),
            separated_pair(
                u32,
                tag(" "),
                alt((
                    value('r', tag("red")),
                    value('g', tag("green")),
                    value('b', tag("blue")),
                )),
            ),
        )(input)?;
        let mut set = Set { r: 0, g: 0, b: 0 };
        for (n, color) in color_vals {
            match color {
                'r' => set.r = n,
                'g' => set.g = n,
                'b' => set.b = n,
                _ => return Err(nom::Err::Error(Error::new("Not a color", ErrorKind::Fail))),
            }
        }
        Ok((remaining, set))
    }
}

pub fn run(input: &str) -> color_eyre::Result<(u64, u64)> {
    Ok((part1(input)? as u64, part2(input)? as u64))
}

fn part1(input: &str) -> color_eyre::Result<u32> {
    let sum: u32 = input
        .par_lines()
        .map(|l| {
            let (_remaining, (n, sets)) = parse::parse_game(l).unwrap();
            if sets.iter().any(|s| s.r > 12 || s.g > 13 || s.b > 14) {
                return 0;
            }
            n
        })
        .sum();
    Ok(sum)
}

fn part2(input: &str) -> color_eyre::Result<u32> {
    let sum: u32 = input
        .par_lines()
        .map(|l| {
            let (_remaining, (_n, sets)) = parse::parse_game(l).unwrap();
            let max = sets.iter().fold(Set { r: 0, g: 0, b: 0 }, |acc, set| Set {
                r: max(acc.r, set.r),
                g: max(acc.g, set.g),
                b: max(acc.b, set.b),
            });
            max.r * max.g * max.b
        })
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {

    use crate::runner::test::{input_test, sample_test};

    use super::*;

    sample_test!(sample_part1, 2, Some(8), None);
    input_test!(part1, 2, Some(2685), None);
    sample_test!(sample_part2, 2, None, Some(2286));

    #[test]
    fn can_parse_color_set() -> color_eyre::Result<()> {
        let input = "4 blue, 16 green, 2 red";
        let (_, set) = parse::parse_set(input)?;
        assert_eq!(4, set.b);
        assert_eq!(16, set.g);
        assert_eq!(2, set.r);
        Ok(())
    }

    #[test]
    fn can_parse_game() -> color_eyre::Result<()> {
        let input = "Game 15: 4 blue, 16 green, 2 red; 5 red, 11 blue, 16 green; 9 green, 11 blue; 10 blue, 6 green, 4 red";
        let (_, (n, sets)) = parse::parse_game(input)?;
        assert_eq!(15, n);
        assert_eq!(4, sets.len());

        Ok(())
    }
}
