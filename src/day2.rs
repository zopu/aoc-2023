use std::cmp::max;

use color_eyre::eyre::eyre;

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
    use nom::multi::separated_list0;
    use nom::sequence::tuple;
    use nom::{combinator::value, sequence::separated_pair, IResult};

    #[derive(Clone, Debug)]
    pub enum Color {
        Red,
        Green,
        Blue,
    }

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
                    value(Color::Red, tag("red")),
                    value(Color::Green, tag("green")),
                    value(Color::Blue, tag("blue")),
                )),
            ),
        )(input)?;
        let mut set = Set { r: 0, g: 0, b: 0 };
        for v in color_vals {
            match v {
                (n, Color::Red) => set.r = n,
                (n, Color::Green) => set.g = n,
                (n, Color::Blue) => set.b = n,
            }
        }
        Ok((remaining, set))
    }
}

pub fn run(input: &str) -> color_eyre::Result<(u32, u32)> {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> color_eyre::Result<u32> {
    let sum: u32 = input
        .lines()
        .map(|l| {
            let parsed = parse::parse_game(l)
                .map_err(|_| eyre!("Parse error"))
                .unwrap();
            for set in parsed.1 .1 {
                if set.r > 12 || set.g > 13 || set.b > 14 {
                    return 0;
                }
            }
            parsed.1 .0
        })
        .sum();
    Ok(sum)
}

fn part2(input: &str) -> color_eyre::Result<u32> {
    let sum: u32 = input
        .lines()
        .map(|l| {
            let parsed = parse::parse_game(l)
                .map_err(|_| eyre!("Parse error"))
                .unwrap();
            let max = parsed
                .1
                 .1
                .iter()
                .fold(Set { r: 0, g: 0, b: 0 }, |acc, set| Set {
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
    use super::*;

    #[test]
    fn test_sample_part1() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/2/sample.txt")?;
        assert_eq!(8, part1(&input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/2/input.txt")?;
        assert_eq!(2685, part1(&input)?);
        Ok(())
    }

    #[test]
    fn test_sample_part2() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/2/sample.txt")?;
        assert_eq!(2286, part2(&input)?);
        Ok(())
    }

    #[test]
    fn can_parse_color_set() -> color_eyre::Result<()> {
        let input = "4 blue, 16 green, 2 red";
        let (_, set) = super::parse::parse_set(input)?;
        assert_eq!(4, set.b);
        assert_eq!(16, set.g);
        assert_eq!(2, set.r);
        Ok(())
    }

    #[test]
    fn can_parse_game() -> color_eyre::Result<()> {
        let input = "Game 15: 4 blue, 16 green, 2 red; 5 red, 11 blue, 16 green; 9 green, 11 blue; 10 blue, 6 green, 4 red";
        let (_, (n, sets)) = super::parse::parse_game(input)?;
        assert_eq!(15, n);
        assert_eq!(4, sets.len());

        Ok(())
    }
}
