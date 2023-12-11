use color_eyre::{eyre::anyhow, Result};
use nom::{bytes::complete::tag, character::complete::i32, multi::separated_list1, IResult};
use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn run(input: &str) -> Result<(u64, u64)> {
    let (p1, p2) = input
        .par_lines()
        .map(|l| {
            let (_, v) = parse_line(l)
                .map_err(|e| anyhow!("Parse error: {}", e))
                .unwrap();
            (find_next(v.iter()), find_next(v.iter().rev()))
        })
        .reduce(
            || (0, 0),
            |(p1sum, p2sum), (p1, p2)| (p1sum + p1, p2sum + p2),
        );
    Ok((p1 as u64, p2 as u64))
}

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(tag(" "), i32)(input)
}

fn find_next<'a>(seq: impl Iterator<Item = &'a i32>) -> i32 {
    let mut v = Vec::with_capacity(20);
    for &n in seq {
        if v.is_empty() {
            v.push(n);
            continue;
        }
        let mut last_value = v[0];
        v[0] = n;
        for i in 1..v.len() {
            (last_value, v[i]) = (v[i], v[i - 1] - last_value);
        }
        let diff = v[v.len() - 1] - last_value;
        if diff != 0 {
            v.push(diff);
        }
    }
    v.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part1, 9, Some(114), None);
    sample_test!(sample_part2, 9, None, Some(2));
    input_test!(part1, 9, Some(1819125966), None);
    input_test!(part2, 9, None, Some(1140));
}
