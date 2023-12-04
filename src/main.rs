use std::time::Instant;

mod runner;

mod day1;
mod day2;
mod day3;
mod day4;

use runner::normal_day;

#[derive(Debug)]
struct AppArgs {
    opt_profile_day: Option<usize>,
    profile_times: usize,
}

const DAYS: [fn() -> color_eyre::Result<()>; 4] = [
    || normal_day(day1::run, 1, 55816, 54980),
    || normal_day(day2::run, 2, 2685, 83707),
    || normal_day(day3::run, 3, 527369, 73074886),
    || normal_day(day4::run, 4, 17782, 8477787),
];

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
