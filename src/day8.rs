use std::collections::HashMap;

use color_eyre::{eyre::anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    character::complete::char,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    L,
    R,
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let parsed = parse(input)?;

    let mut count = 0;
    let mut location = parsed.names_map["AAA"];
    let end = parsed.names_map["ZZZ"];
    // println!("Names map: {:?}", names_map);
    for d in parsed.directions.iter().cycle() {
        // println!("Visiting location {}", location);
        if location == end {
            break;
        }
        let routes = parsed.nodes_map[&location];
        location = match d {
            Direction::L => routes.0,
            Direction::R => routes.1,
        };
        count += 1;
    }
    Ok((count, 0))
}

struct ParseOutput<'a> {
    directions: Vec<Direction>,
    names_map: HashMap<&'a str, u16>,
    nodes_map: HashMap<u16, (u16, u16)>,
}

fn parse(input: &str) -> Result<ParseOutput> {
    let mut it = input.lines();
    let directions_input = it.next().unwrap();
    let directions: Vec<Direction> = directions_input
        .chars()
        .map(|c| match c {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("Invalid direction"),
        })
        .collect();
    it.next();

    let mut names_count = 0;
    let mut nodes_map = HashMap::new();
    let mut names_map = HashMap::new();
    for line in it {
        let (_, (a, b, c)) = parse_node(line).map_err(|e| anyhow!("Node parse error {:?}", e))?;
        for name in [a, b, c] {
            if !names_map.contains_key(name) {
                names_count += 1;
                names_map.insert(name, names_count);
            }
        }
        let (source, l, r) = (names_map[a], names_map[b], names_map[c]);
        nodes_map.insert(source, (l, r));
    }
    Ok(ParseOutput {
        directions,
        names_map,
        nodes_map,
    })
}

pub fn parse_node(line: &str) -> IResult<&str, (&str, &str, &str)> {
    let (_remaining, (a, _, (b, c))) = tuple((
        alpha1,
        tag(" = "),
        delimited(
            char('('),
            separated_pair(alpha1, tag(", "), alpha1),
            char(')'),
        ),
    ))(line)?;
    Ok(("", (a, b, c)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::{test_input, test_sample};

    test_sample!(sample_part1, 8, Some(6), None);
    test_input!(part1, 8, Some(15517), None);

    #[test]
    fn parse_node_line() {
        let line = "CCC = (ZZZ, GGG)";
        let (_, (a, b, c)) = parse_node(line).unwrap();
        assert_eq!("CCC", a);
        assert_eq!("ZZZ", b);
        assert_eq!("GGG", c);
    }
}
