pub fn test_one_file<T>(
    f: fn(&str) -> color_eyre::Result<(T, T)>,
    day: usize,
    filename: &str,
    part1: Option<T>,
    part2: Option<T>,
) -> color_eyre::Result<()>
where
    T: PartialEq,
    T: std::fmt::Debug,
{
    let input = std::fs::read_to_string(format!("inputs/{}/{}", day, filename))?;
    let (p1, p2) = f(&input)?;
    if let Some(p1_ans) = part1 {
        assert_eq!(p1_ans, p1);
    }
    if let Some(p2_ans) = part2 {
        assert_eq!(p2_ans, p2);
    }
    Ok(())
}

pub fn normal_day(
    f: fn(&str) -> color_eyre::Result<(u64, u64)>,
    day: usize,
    part1: u64,
    part2: u64,
) -> color_eyre::Result<()> {
    test_one_file(f, day, "input.txt", Some(part1), Some(part2))
}

#[cfg(test)]
pub mod test {
    macro_rules! file_test {
        ( $name: ident, $day: literal, $filename: literal, $part1: expr, $part2: expr) => {
            #[test]
            fn $name() {
                crate::runner::test_one_file(run, $day, $filename, $part1, $part2).unwrap()
            }
        };
    }
    pub(crate) use file_test;

    macro_rules! sample_test {
        ( $name: ident, $day: literal, $part1: expr, $part2: expr) => {
            crate::runner::test::file_test!($name, $day, "sample.txt", $part1, $part2);
        };
    }
    pub(crate) use sample_test;

    #[cfg(test)]
    macro_rules! input_test {
        ( $name: ident, $day: literal, $part1: expr, $part2: expr) => {
            crate::runner::test::file_test!($name, $day, "input.txt", $part1, $part2);
        };
    }
    pub(crate) use input_test;
}
