use color_eyre::eyre::anyhow;
use color_eyre::Result;
use nom::character::complete::{multispace1, newline, u32};
use nom::sequence::separated_pair;
use nom::{
    bytes::complete::tag,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

pub fn run(input: &str) -> Result<(u64, u64)> {
    Ok((part1(input)? as u64, 0))
}

fn part1(input: &str) -> Result<u32> {
    let (_remaining, (times, distances)) =
        parse(input).map_err(|e| anyhow!("Parse error: {}", e))?;
    let mut product = 1;
    for (t, d) in times.iter().zip(distances.iter()) {
        let n = solve(*t, *d);
        product *= n;
    }
    Ok(product)
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(
        preceded(
            tuple((tag("Time:"), multispace1)),
            separated_list1(multispace1, u32),
        ),
        newline,
        preceded(
            tuple((tag("Distance:"), multispace1)),
            separated_list1(multispace1, u32),
        ),
    )(input)
}

fn solve(t: u32, d: u32) -> u32 {
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

    top_floor as u32 - bottom_ceil as u32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        day6::solve,
        runner::{test_input, test_sample},
    };

    test_sample!(sample_part1, 6, Some(288), None);
    test_input!(part1, 6, Some(512295), None);

    #[test]
    fn can_solve_problem() {
        assert_eq!(4, solve(7, 9));
        assert_eq!(8, solve(15, 40));
        assert_eq!(9, solve(30, 200));
    }
}
