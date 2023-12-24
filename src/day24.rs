use color_eyre::Result;

pub fn run(_input: &str) -> Result<(u64, u64)> {
    Ok((0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part1, 24, Some(0), None);
    input_test!(part1, 24, Some(0), None);
}
