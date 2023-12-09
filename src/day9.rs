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
            let p1 = find_next(v.iter());
            let p2 = find_next(v.iter().rev());
            (p1, p2)
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
    let next_diffs = |mut v: Vec<i32>, n: &i32| -> Vec<i32> {
        if v.is_empty() {
            v.push(*n);
            return v;
        };
        let mut last_v_i = v[0];
        v[0] = *n;
        for i in 1..v.len() {
            (last_v_i, v[i]) = (v[i], v[i - 1] - last_v_i);
        }
        let diff = v[v.len() - 1] - last_v_i;
        if diff != 0 {
            v.push(diff);
        }
        v
    };
    let diffs = seq.fold(Vec::with_capacity(20), next_diffs);
    diffs.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::{test_input, test_sample};

    test_sample!(sample_part1, 9, Some(114), None);
    test_sample!(sample_part2, 9, None, Some(2));
    test_input!(part1, 9, Some(1819125966), None);
    test_input!(part2, 9, None, Some(1140));
}
