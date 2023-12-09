use color_eyre::Result;

pub fn run(input: &str) -> Result<(u64, u64)> {
    Ok((part1(&input)?, 0))
}

fn part1(_input: &str) -> Result<u64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test_sample;

    test_sample!(sample_part1, 9, Some(0), None);
}
