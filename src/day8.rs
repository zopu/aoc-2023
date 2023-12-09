use color_eyre::{eyre::anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::alphanumeric1,
    character::complete::char,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use num::Integer;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    L,
    R,
}

type Node = u16;
type StepCount = u64;

pub fn run(input: &str) -> Result<(u64, u64)> {
    let parsed = parse(input)?;

    let p1_start = parsed.names_map["AAA"];
    let (p1, p2) = rayon::join(
        || part1(&parsed.directions, p1_start, &parsed.nodes),
        || part2(&parsed),
    );
    Ok((p1?, p2?))
}

fn part1(directions: &[Direction], start: u16, nodes: &[NodeMapEntry]) -> Result<u64> {
    find_first_ending(directions, start, nodes)
}

fn part2(parsed: &ParseOutput) -> Result<u64> {
    // This *happens* to work on both the sample data and input data,
    // where the initial offset and loop lengths are all the same size,
    // but can't be assumed to generally work.
    let ghost_moves: Vec<StepCount> = parsed
        .ghost_starts
        .par_iter()
        .map(|ghost_location| find_first_ending(&parsed.directions, *ghost_location, &parsed.nodes))
        .collect::<Result<Vec<StepCount>>>()?;
    let answer: u64 = ghost_moves.iter().cloned().fold(1u64, |p, a| p.lcm(&a));
    Ok(answer)
}

fn find_first_ending(
    directions: &[Direction],
    start: u16,
    nodes: &[NodeMapEntry],
) -> Result<StepCount> {
    let mut location = start;

    for (i, d) in directions.iter().cycle().enumerate() {
        let entry = &nodes[location as usize];
        if entry.is_end {
            return Ok(i as u64);
        }
        location = match d {
            Direction::L => entry.l,
            Direction::R => entry.r,
        };
    }
    Err(anyhow!("Couldn't find a loop!"))
}

#[derive(Clone, Debug)]
struct NodeMapEntry {
    l: Node,
    r: Node,
    is_end: bool,
}

struct ParseOutput<'a> {
    directions: Vec<Direction>,
    names_map: HashMap<&'a str, u16>,
    nodes: Vec<NodeMapEntry>,
    ghost_starts: Vec<u16>,
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
    let mut names_map = HashMap::new();
    let mut ghost_starts = Vec::new();
    let mut ghost_ends = BTreeSet::new();
    let mut nodes = Vec::new();
    for line in it {
        let (_, (a, b, c)) = parse_node(line).map_err(|e| anyhow!("Node parse error {:?}", e))?;
        for name in [a, b, c] {
            if !names_map.contains_key(name) {
                names_map.insert(name, names_count);
                names_count += 1;
                nodes.push(NodeMapEntry {
                    is_end: false,
                    l: 0,
                    r: 0,
                });
            }
        }
        let (source, l, r) = (names_map[a], names_map[b], names_map[c]);

        let mut is_end = false;
        match a.chars().nth(2) {
            Some('A') => {
                ghost_starts.push(source);
            }
            Some('Z') => {
                ghost_ends.insert(source);
                is_end = true;
            }
            _ => {}
        }
        nodes[source as usize] = NodeMapEntry { is_end, l, r };
    }
    Ok(ParseOutput {
        directions,
        names_map,
        nodes,
        ghost_starts,
    })
}

pub fn parse_node(line: &str) -> IResult<&str, (&str, &str, &str)> {
    let (_remaining, (a, _, (b, c))) = tuple((
        alphanumeric1,
        tag(" = "),
        delimited(
            char('('),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
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
    test_input!(part2, 8, None, Some(14935034899483));

    #[test]
    fn sample_part2() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/8/sample_part2.txt")?;
        let parsed = parse(&input)?;
        assert_eq!(6, super::part2(&parsed)?);
        Ok(())
    }

    #[test]
    fn parse_node_line() {
        let line = "CCC = (ZZZ, GGG)";
        let (_, (a, b, c)) = parse_node(line).unwrap();
        assert_eq!("CCC", a);
        assert_eq!("ZZZ", b);
        assert_eq!("GGG", c);
    }
}
