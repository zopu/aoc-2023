use color_eyre::eyre::eyre;

#[derive(Clone, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

mod parse {
    use super::Color;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::u32;
    use nom::multi::separated_list0;
    use nom::sequence::tuple;
    use nom::{combinator::value, sequence::separated_pair, IResult};

    type ParsedGame = Vec<Vec<(u32, Color)>>;

    pub fn parse_game(input: &str) -> IResult<&str, (u32, ParsedGame)> {
        let (remaining, (_, num, _)) = tuple((tag("Game "), u32, tag(": ")))(input)?;
        let (remaining, game_result) = separated_list0(tag("; "), parse_set)(remaining)?;
        Ok((remaining, (num, game_result)))
    }

    pub fn parse_set(input: &str) -> IResult<&str, Vec<(u32, Color)>> {
        separated_list0(
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
        )(input)
    }
}

pub fn run(input: &str) -> color_eyre::Result<(u32, u32)> {
    Ok((part1(input)?, 0))
}

fn part1(input: &str) -> color_eyre::Result<u32> {
    let sum: u32 = input
        .lines()
        .map(|l| {
            let parsed = parse::parse_game(l)
                .map_err(|_| eyre!("Parse error"))
                .unwrap();
            for set in parsed.1 .1 {
                for color_val in set {
                    match color_val {
                        (n, Color::Red) if n > 12 => return 0,
                        (n, Color::Green) if n > 13 => return 0,
                        (n, Color::Blue) if n > 14 => return 0,
                        _ => {}
                    }
                }
            }
            parsed.1 .0
        })
        .sum();

    println!("{:?}", sum);
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
    fn can_parse_color_set() -> color_eyre::Result<()> {
        let input = "4 blue, 16 green, 2 red";
        let (_, v) = super::parse::parse_set(input)?;
        assert_eq!(3, v.len());
        assert_eq!(4, v[0].0);
        assert!(matches!(v[0].1, Color::Blue));
        assert_eq!(16, v[1].0);
        assert!(matches!(v[1].1, Color::Green));
        assert_eq!(2, v[2].0);
        assert!(matches!(v[2].1, Color::Red));

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
