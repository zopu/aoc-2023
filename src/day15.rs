use color_eyre::Result;

pub fn run(input: &str) -> Result<(u64, u64)> {
    let p1: u64 = input.split(',').map(|s| hash(0, s.chars())).sum();
    Ok((p1, 0))
}

fn hash(mut current: u64, input: impl Iterator<Item = char>) -> u64 {
    for c in input {
        let ascii = c as u8;
        current += ascii as u64;
        current *= 17;
        current %= 256;
    }
    current
}

#[cfg(test)]
mod tests {
    use crate::runner::test::{input_test, sample_test};

    use super::*;

    sample_test!(sample_part1, 15, Some(1320), None);
    input_test!(part1, 15, Some(516657), None);

    #[test]
    fn hashes_hash() {
        assert_eq!(52, hash(0, "HASH".chars()));
    }
}
