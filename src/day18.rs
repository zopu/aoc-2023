use std::cmp::{max, min};

use color_eyre::{eyre::anyhow, Result};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    IResult,
};

pub fn run(input: &str) -> Result<(u64, u64)> {
    let lines: Vec<Line> = input
        .lines()
        .map(parse_line)
        .map(|r| r.map(|(_, l)| l))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| anyhow!("Parse error: {}", e))?;

    // Follow lines starting from (0, 0) and map lines to cartesian coordinates
    let mut coords: Vec<(i32, i32)> = vec![];
    let mut pos = (0, 0);
    let mut last_dir = lines[0].dir;
    let mut l_r_counts = (0, 0);
    coords.push(pos);
    for line in &lines {
        match line.dir {
            Dir::Up => pos.1 -= line.len as i32,
            Dir::Down => pos.1 += line.len as i32,
            Dir::Left => pos.0 -= line.len as i32,
            Dir::Right => pos.0 += line.len as i32,
        }
        coords.push(pos);

        // Record how many times we turn left/right
        // Later we'll need this to add the correct "outside" area to the polygon
        match (last_dir, line.dir) {
            (Dir::Up, Dir::Left)
            | (Dir::Down, Dir::Right)
            | (Dir::Left, Dir::Down)
            | (Dir::Right, Dir::Up) => l_r_counts.0 += 1,
            (a, b) if a == b => {}
            _ => l_r_counts.1 += 1,
        }
        last_dir = line.dir;
    }

    let mut shoelace_sum: i64 = 0;
    let mut prev = coords.last().unwrap();
    for coord in &coords {
        shoelace_sum += prev.0 as i64 * coord.1 as i64 - prev.1 as i64 * coord.0 as i64;
        prev = coord;
    }
    let enclosed_area = shoelace_sum.unsigned_abs() / 2;

    // Extra 0.75 for final turn. NB we're assuming the end of the line is always a corner
    let corner_area = max(l_r_counts.0, l_r_counts.1) as f64 * 0.75
        + min(l_r_counts.0, l_r_counts.1) as f64 * 0.25
        + 0.75;
    let outside_area = lines
        .iter()
        .map(|l| (l.len as f64 - 1.0) / 2.0)
        .sum::<f64>()
        + corner_area as f64;
    let p1 = enclosed_area + outside_area as u64;
    Ok((p1, 0))
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Line {
    dir: Dir,
    len: u8,
    _color: u32, // argb
}

// Parse a line like "L 4 (#38ce32)""
fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, dir) = nom::branch::alt((tag("U"), tag("D"), tag("L"), tag("R")))(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, len) = digit1(input)?;
    let (input, _) = tag(" (")(input)?;
    let (input, color) = take_until(")")(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((
        input,
        Line {
            dir: match dir {
                "U" => Dir::Up,
                "D" => Dir::Down,
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => unreachable!(),
            },
            len: len.parse().unwrap(),
            _color: u32::from_str_radix(&color[1..], 16).unwrap(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part1, 18, Some(62), None);
    input_test!(part1, 18, Some(31171), None);

    #[test]
    fn can_parse_line() {
        let input = "U 4 (#38ce32)";
        let (_, line) = parse_line(input).unwrap();
        assert_eq!(line.dir, Dir::Up);
        assert_eq!(line.len, 4);
        assert_eq!(line._color, 0x38ce32);
    }
}
