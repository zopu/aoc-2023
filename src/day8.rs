use color_eyre::Result;

pub fn run(_input: &str) -> Result<(u64, u64)> {
    Ok((0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test_sample;

    test_sample!(sample_part1, 8, Some(0), None);
}
