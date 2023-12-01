use std::time::Instant;

mod day1;

fn main() -> color_eyre::Result<()> {
    let start = Instant::now();
    let day1_input = std::fs::read_to_string("inputs/1/input.txt")?;
    let (p1, p2) = day1::day1(&day1_input)?;
    assert_eq!(55816, p1);
    assert_eq!(54980, p2);

    let duration = Instant::now().duration_since(start);
    println!("{:?} us day 1 runtime", duration.as_micros());

    Ok(())
}
