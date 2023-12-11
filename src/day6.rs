use color_eyre::eyre::anyhow;
use color_eyre::Result;
use itertools::Itertools;
use nom::character::complete::{alpha1, multispace1, newline, u32};
use nom::{
    bytes::complete::tag,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

pub fn run(input: &str) -> Result<(u64, u64)> {
    Ok((part1(input)? as u64, part2(input)?))
}

fn part1(input: &str) -> Result<u64> {
    let (_remaining, (times, distances)) =
        parse_part1(input).map_err(|e| anyhow!("Parse error: {}", e))?;
    let mut product = 1;
    for (t, d) in times.iter().zip(distances.iter()) {
        let n = solve(*t as u64, *d as u64);
        product *= n;
    }
    Ok(product)
}

fn part2(input: &str) -> Result<u64> {
    let (t, d) = parse_part2(input);
    Ok(solve(t, d))
}

fn parse_part1(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (r, v) = separated_list1(
        newline,
        preceded(
            tuple((alpha1, tag(":"), multispace1)),
            separated_list1(multispace1, u32),
        ),
    )(input)?;
    Ok((r, v.into_iter().next_tuple().unwrap()))
}

fn parse_part2(input: &str) -> (u64, u64) {
    input
        .lines()
        .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect())
        .map(|l: String| l.parse::<u64>().unwrap())
        .next_tuple()
        .unwrap()
}

fn solve(t: u64, d: u64) -> u64 {
    // Quuadratic formula!
    let t = t as f64;
    let d = d as f64;
    let half_t = t / 2.0;
    let plus_minus = (t.powi(2) - 4.0 * d).sqrt() / 2.0;
    let bottom = half_t - plus_minus;
    let top = half_t + plus_minus;

    // We need to beat the record and not just match
    let mut top_floor = top.floor();
    if top_floor == top {
        top_floor -= 1.0;
    }
    let mut bottom_ceil = bottom.ceil();
    if bottom_ceil == bottom {
        bottom_ceil += 1.0;
    }

    top_floor as u64 - bottom_ceil as u64 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        day6::solve,
        runner::test::{input_test, sample_test},
    };

    sample_test!(sample_part1, 6, Some(288), None);
    sample_test!(sample_part2, 6, None, Some(71503));
    input_test!(part1, 6, Some(512295), None);
    input_test!(part2, 6, None, Some(36530883));

    #[test]
    fn can_solve_problem() {
        assert_eq!(4, solve(7, 9));
        assert_eq!(8, solve(15, 40));
        assert_eq!(9, solve(30, 200));
    }
}
