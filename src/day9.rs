use color_eyre::{eyre::anyhow, Result};
use nom::{bytes::complete::tag, character::complete::i32, multi::separated_list1, IResult};

pub fn run(input: &str) -> Result<(u64, u64)> {
    let (p1, p2) = input
        .lines()
        .map(|l| {
            let (_, mut v) = parse_line(l)
                .map_err(|e| anyhow!("Parse error: {}", e))
                .unwrap();
            let p1 = find_next(&v);
            v.reverse();
            let p2 = find_next(&v);
            (p1, p2)
        })
        .reduce(|(p1sum, p2sum), (p1, p2)| (p1sum + p1, p2sum + p2))
        .unwrap();
    Ok((p1 as u64, p2 as u64))
}

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(tag(" "), i32)(input)
}

fn find_next(seq: &[i32]) -> i32 {
    let next_diffs = |v: Vec<i32>, n| -> Vec<i32> {
        let mut next_v = vec![n];
        for i in 1..v.len() + 1 {
            next_v.push(next_v[i - 1] - v[i - 1]);
        }
        if next_v[v.len()] == 0 {
            next_v.pop();
        }
        next_v
    };
    let diffs = seq.iter().cloned().fold(Vec::new(), next_diffs);
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
