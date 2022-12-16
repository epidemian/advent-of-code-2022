use std::{env, fs, process::ExitCode};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> ExitCode {
    let args: Vec<_> = env::args().collect();
    let days = [day1::run, day2::run, day3::run, day4::run, day5::run];

    let run_single_day = |day_num: usize| {
        let filename = format!("inputs/day{}.txt", day_num);
        match fs::read_to_string(&filename) {
            Ok(input) => {
                let (ans1, ans2) = days[day_num - 1](&input);
                println!("{}\n{}", ans1, ans2);
                Ok(())
            }
            Err(err) => {
                eprintln!("Error reading {}: {}", filename, err);
                Err(())
            }
        }
    };

    match args.len() {
        1 => {
            for day_num in 1..=days.len() {
                println!("Day {}", day_num);
                if run_single_day(day_num).is_err() {
                    return ExitCode::FAILURE;
                };
            }
        }
        2 => {
            let Ok(day_num) = args[1].parse::<usize>() else {
                eprintln!("Invalid day number");
                return ExitCode::FAILURE
            };
            if day_num < 1 || day_num > days.len() {
                eprintln!("Day number out of range");
                return ExitCode::FAILURE;
            }

            if run_single_day(day_num).is_err() {
                return ExitCode::FAILURE;
            };
        }
        _ => {
            eprintln!("Usage: {} [day_number]", args[0]);
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
