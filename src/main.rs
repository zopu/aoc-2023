const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> color_eyre::Result<()> {
    let day1_input = std::fs::read_to_string("inputs/1/input.txt")?;
    let mut sum: u32 = 0;
    for l in day1_input.lines() {
        let mut digits = l.chars().filter(|c| c.is_ascii_digit());
        let first = digits.next();
        let last = digits.last().or(first);

        let mut s = first.unwrap().to_string();
        s.push(last.unwrap());

        let num: u32 = s.parse().unwrap();
        sum += num;
    }
    println!("Part 1: {:?}", sum);

    let mut sum: u32 = 0;
    for l in day1_input.lines() {
        let mut s = l;
        let mut digits = Vec::<u32>::new();
        while !s.is_empty() {
            if let Some(d) = leading_digit(s) {
                digits.push(d);
            }
            s = &s[1..];
        }
        let n = digits[0] * 10 + digits[digits.len() - 1];
        sum += n;
    }
    println!("Part 2: {:?}", sum);

    Ok(())
}

fn leading_digit(s: &str) -> Option<u32> {
    if s.is_empty() {
        return None;
    };
    let first = s.chars().next()?;
    if first.is_ascii_digit() {
        return Some(u32::from(first) - '0' as u32);
    };
    for (i, d) in DIGITS.iter().enumerate().skip(1) {
        if s.starts_with(d) {
            return Some(i as u32);
        }
    }
    None
}
