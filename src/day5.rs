use std::cmp::{max, min};
use std::ops::Range;

use color_eyre::eyre::anyhow;
use color_eyre::Result;
use nom::branch::alt;
use nom::character::complete::{alpha1, multispace1, newline, u64};
use nom::combinator::map;
use nom::multi::{many0, separated_list0};
use nom::sequence::tuple;
use nom::{bytes::complete::tag, multi::separated_list1, sequence::preceded, IResult};

pub fn run(input: &str) -> Result<(u64, u64)> {
    let (_, (seeds, mappings)) = parse(input).map_err(|e| anyhow!("Parse error: {}", e))?;
    Ok((part1(&seeds, &mappings)?, part2(&seeds, &mappings)?))
}

fn part1(seeds: &[u64], mappings: &[Mapping]) -> Result<u64> {
    let mut vals = Vec::from(seeds);
    for mapping in mappings {
        vals = vals.iter().map(|v| mapping.apply(*v)).collect();
    }
    Ok(*vals.iter().min().unwrap())
}

fn part2(seeds: &[u64], mappings: &[Mapping]) -> Result<u64> {
    let mut ranges: Vec<Range<u64>> = seeds
        .chunks(2)
        .map(|chunk| Range {
            start: chunk[0],
            end: chunk[0] + chunk[1],
        })
        .collect();
    for mapping in mappings {
        ranges = ranges
            .iter()
            .flat_map(|r| mapping.apply_to_range(r).into_iter())
            .collect();
    }

    Ok(ranges.iter().map(|r| r.start).min().unwrap())
}

#[derive(Debug)]
struct Mapping {
    maps: Vec<(u64, u64, u64)>,
}

impl Mapping {
    pub fn apply(&self, n: u64) -> u64 {
        for map in &self.maps {
            if n >= map.1 && n < map.1 + map.2 {
                return (n as i64 + (map.0 as i64 - map.1 as i64)) as u64;
            }
        }
        n
    }

    fn apply_submap(n: u64, map: &(u64, u64, u64)) -> u64 {
        (n as i64 + (map.0 as i64 - map.1 as i64)) as u64
    }

    pub fn apply_to_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        let mut output_ranges: Vec<Range<u64>> = Vec::new();
        let mut intersections: Vec<Range<u64>> = Vec::new();
        for map in &self.maps {
            let map_range = Range {
                start: map.1,
                end: map.1 + map.2,
            };
            if let Some(intersection) = intersection(range, &map_range) {
                let mapped_output = Range {
                    start: Mapping::apply_submap(intersection.start, map),
                    end: Mapping::apply_submap(intersection.end, map),
                };
                output_ranges.push(mapped_output);
                intersections.push(intersection);
            }
        }
        output_ranges.extend(remove_ranges_from_range(range, intersections));
        output_ranges
    }
}

fn intersection(r1: &Range<u64>, r2: &Range<u64>) -> Option<Range<u64>> {
    let start = max(r1.start, r2.start);
    let end = min(r1.end, r2.end);
    if start <= end {
        Some(Range { start, end })
    } else {
        None
    }
}

fn remove_ranges_from_range(range: &Range<u64>, mut to_remove: Vec<Range<u64>>) -> Vec<Range<u64>> {
    to_remove.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
    let mut output_ranges: Vec<Range<u64>> = Vec::new();
    let mut pos = range.start;
    for r in to_remove.iter() {
        if r.start > pos {
            output_ranges.push(Range {
                start: pos,
                end: r.start,
            });
            pos = r.end;
            continue;
        }
        if r.end > pos {
            pos = r.end;
        }
    }
    if pos < range.end {
        output_ranges.push(Range {
            start: pos,
            end: range.end,
        });
    }
    output_ranges
}

fn parse_one_mapping(input: &str) -> IResult<&str, Mapping> {
    let (remaining, _header) =
        tuple((many0(alt((alpha1, tag("-"), tag(" ")))), tag(":\n")))(input)?;
    let (remaining, vec_tuples) = separated_list1(
        newline,
        map(
            tuple((u64, tag(" "), u64, tag(" "), u64)),
            |(a, _, b, _, c)| (a, b, c),
        ),
    )(remaining)?;
    Ok((remaining, Mapping { maps: vec_tuples }))
}

fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<Mapping>)> {
    let (remaining, seeds) = preceded(tag("seeds: "), separated_list1(tag(" "), u64))(input)?;
    let (remaining, _) = many0(newline)(remaining)?;
    let (remaining, mappings) = separated_list0(multispace1, parse_one_mapping)(remaining)?;
    Ok((remaining, (seeds, mappings)))
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::anyhow;

    use super::*;
    use crate::runner::{test_input, test_sample};

    test_sample!(sample_part1, 5, Some(35), None);
    test_sample!(sample_part2, 5, None, Some(46));
    test_input!(part1, 5, Some(251346198), None);
    test_input!(part2, 5, None, Some(72263011));

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
