use std::time::Instant;

mod day1;
mod day2;
mod day3;

#[derive(Debug)]
struct AppArgs {
    opt_profile_day: Option<usize>,
    profile_times: usize,
}

fn day1() -> color_eyre::Result<()> {
    let day1_input = std::fs::read_to_string("inputs/1/input.txt")?;
    let (p1, p2) = day1::run(&day1_input)?;
    assert_eq!(55816, p1);
    assert_eq!(54980, p2);
    Ok(())
}

fn day2() -> color_eyre::Result<()> {
    let day2_input = std::fs::read_to_string("inputs/2/input.txt")?;
    let (p1, p2) = day2::run(&day2_input)?;
    assert_eq!(2685, p1);
    assert_eq!(83707, p2);
    Ok(())
}

fn day3() -> color_eyre::Result<()> {
    let day3_input = std::fs::read_to_string("inputs/3/input.txt")?;
    let (_p1, _p2) = day3::run(&day3_input)?;
    Ok(())
}

const DAYS: [fn() -> color_eyre::Result<()>; 3] = [day1, day2, day3];

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    if let Some(d) = args.opt_profile_day {
        profile_one_day(d, args.profile_times)?;
    } else {
        run_all_days()?;
    }

    Ok(())
}

fn run_all_days() -> color_eyre::Result<()> {
    let start = Instant::now();
    for (i, d) in DAYS.iter().enumerate() {
        let day_start = Instant::now();
        d()?;
        let duration = Instant::now().duration_since(day_start);
        println!("{:?} us day {} runtime", duration.as_micros(), i + 1);
    }

    let duration = Instant::now().duration_since(start);
    let budget = 1000 - duration.as_millis();
    println!(
        "Total time: {}us. Remaining time budget: {}ms. {}ms/day avg",
        duration.as_micros(),
        budget,
        budget / 23
    );
    Ok(())
}

fn profile_one_day(day: usize, times: usize) -> color_eyre::Result<()> {
    println!("Profiling running day {} x{}:", day, times);
    let day_start = Instant::now();
    for _ in 0..times {
        DAYS[day - 1]()?;
    }
    let duration = Instant::now().duration_since(day_start);
    println!(
        "{:?}ms day {} total runtime for {} runs. {}us avg",
        duration.as_millis(),
        day,
        times,
        duration.as_micros() / times as u128
    );
    Ok(())
}

fn parse_args() -> color_eyre::Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    let args = AppArgs {
        opt_profile_day: pargs.opt_value_from_str("--profile-day")?,
        profile_times: pargs.opt_value_from_str("--profile-times")?.unwrap_or(10),
    };

    Ok(args)
}
