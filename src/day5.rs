use color_eyre::eyre::anyhow;
use color_eyre::Result;
use nom::branch::alt;
use nom::character::complete::{alpha1, multispace1, newline, u32};
use nom::combinator::map;
use nom::multi::{many0, separated_list0};
use nom::sequence::tuple;
use nom::{bytes::complete::tag, multi::separated_list1, sequence::preceded, IResult};

pub fn run(input: &str) -> Result<(u64, u64)> {
    Ok((part1(input)? as u64, 0))
}

fn part1(input: &str) -> Result<u32> {
    let (_, (seeds, mappings)) = parse(input).map_err(|e| anyhow!("Parse error: {}", e))?;
    let mut vals = seeds.clone();
    for mapping in mappings {
        vals = vals.iter().map(|v| mapping.apply(*v)).collect();
    }
    Ok(*vals.iter().min().unwrap())
}

#[derive(Debug)]
struct Mapping {
    maps: Vec<(u32, u32, u32)>,
}

impl Mapping {
    pub fn apply(&self, n: u32) -> u32 {
        for map in &self.maps {
            if n >= map.1 && n < map.1 + map.2 {
                return (n as i64 + (map.0 as i64 - map.1 as i64)) as u32;
            }
        }
        n
    }
}

fn parse_one_mapping(input: &str) -> IResult<&str, Mapping> {
    let (remaining, _header) =
        tuple((many0(alt((alpha1, tag("-"), tag(" ")))), tag(":\n")))(input)?;
    let (remaining, vec_tuples) = separated_list1(
        newline,
        map(
            tuple((u32, tag(" "), u32, tag(" "), u32)),
            |(a, _, b, _, c)| (a, b, c),
        ),
    )(remaining)?;
    Ok((remaining, Mapping { maps: vec_tuples }))
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<Mapping>)> {
    let (remaining, seeds) = preceded(tag("seeds: "), separated_list1(tag(" "), u32))(input)?;
    let (remaining, _) = many0(newline)(remaining)?;
    let (remaining, mappings) = separated_list0(multispace1, parse_one_mapping)(remaining)?;
    Ok((remaining, (seeds, mappings)))
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::anyhow;

    use super::*;
    use crate::runner::test_sample;

    test_sample!(sample_part1, 5, Some(35), None);

    #[test]
    fn can_parse_sample() -> Result<()> {
        let input = std::fs::read_to_string("inputs/5/sample.txt")?;
        let (_remaining, (seeds, mappings)) = parse(&input).map_err(|_| anyhow!("parse error"))?;
        assert_eq!(4, seeds.len());
        assert_eq!(55, seeds[2]);

        assert_eq!(7, mappings.len());
        println!("{:?}", mappings);
        Ok(())
    }
}
