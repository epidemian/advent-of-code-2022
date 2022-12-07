use std::{env, process::ExitCode};

mod day1;
mod day2;
mod day3;

fn main() -> ExitCode {
    let args: Vec<_> = env::args().collect();
    let days = [day1::run, day2::run, day3::run];

    match args.len() {
        1 => {
            for (day_num, day_fn) in days.iter().enumerate() {
                println!("Day {}", day_num + 1);
                day_fn();
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

            days[day_num - 1]();
        }
        _ => {
            eprintln!("Usage: {} [day_number]", args[0]);
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
