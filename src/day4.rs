pub fn run(input: &str) -> color_eyre::Result<(u32, u32)> {
    Ok((part1(input)?, 0))
}

fn part1(input: &str) -> color_eyre::Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() -> color_eyre::Result<()> {
        let input = std::fs::read_to_string("inputs/4/sample.txt")?;
        assert_eq!(0, part1(&input)?);
        Ok(())
    }
}
