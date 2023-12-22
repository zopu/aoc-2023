use std::time::{Duration, Instant};

mod runner;
mod symbol_table;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod grid;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use runner::normal_day;

#[derive(Debug)]
struct AppArgs {
    opt_profile_day: Option<usize>,
    profile_times: usize,
    parallel: bool,
}

const DAYS: [fn() -> color_eyre::Result<()>; 22] = [
    || normal_day(day1::run, 1, 55816, 54980),
    || normal_day(day2::run, 2, 2685, 83707),
    || normal_day(day3::run, 3, 527369, 73074886),
    || normal_day(day4::run, 4, 17782, 8477787),
    || normal_day(day5::run, 5, 251346198, 72263011),
    || normal_day(day6::run, 6, 512295, 36530883),
    || normal_day(day7::run, 7, 250347426, 251224870),
    || normal_day(day8::run, 8, 15517, 14935034899483),
    || normal_day(day9::run, 9, 1819125966, 1140),
    || normal_day(day10::run, 10, 6923, 529),
    || normal_day(day11::run, 11, 9329143, 710674907809),
    || normal_day(day12::run, 12, 6852, 8475948826693),
    || normal_day(day13::run, 13, 37718, 40995),
    || normal_day(day14::run, 14, 102497, 105008),
    || normal_day(day15::run, 15, 516657, 210906),
    || normal_day(day16::run, 16, 6514, 8089),
    || normal_day(day17::run, 17, 1155, 1283),
    || normal_day(day18::run, 18, 31171, 131431655002266),
    || normal_day(day19::run, 19, 374873, 122112157518711),
    || normal_day(day20::run, 20, 743090292, 241528184647003),
    || normal_day(day21::run, 21, 0, 0),
    || normal_day(day22::run, 22, 0, 0),
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
        run_all_days(args.parallel)?;
    }

    Ok(())
}

fn run_all_days(parallel: bool) -> color_eyre::Result<()> {
    let start = Instant::now();
    if parallel {
        DAYS.par_iter().for_each(|d| {
            d().unwrap();
        });
    } else {
        // Run in serial
        for (i, d) in DAYS.iter().enumerate() {
            let day_start = Instant::now();
            d()?;
            println!("Day {}:\t{}", i + 1, format_runtime_elapsed(&day_start));
        }
    }

    let duration = Instant::now().duration_since(start);
    let budget = Duration::from_secs(1) - duration;
    println!(
        "Total time: {}. Remaining time budget: {}. {}/day avg remaining",
        format_runtime_duration(&duration),
        format_runtime_duration(&budget),
        format_runtime_duration(&(budget / (25 - DAYS.len()) as u32)),
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
        parallel: pargs.contains("--parallel"),
    };

    Ok(args)
}

fn format_runtime_elapsed(instant: &Instant) -> String {
    format_runtime_duration(&instant.elapsed())
}

fn format_runtime_duration(d: &Duration) -> String {
    if d.as_micros() < 5000 {
        format!("{}us", d.as_micros())
    } else if d.as_millis() < 2000 {
        format!("{}ms", d.as_millis())
    } else {
        format!("{}s", d.as_secs())
    }
}
