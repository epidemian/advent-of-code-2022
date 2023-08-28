use rayon::prelude::*;
use std::{env, fs, process::ExitCode, time};

mod day_01_calorie_counting;
mod day_02_rock_paper_scissors;
mod day_03_rucksack_reorganization;
mod day_04_camp_cleanup;
mod day_05_supply_stacks;
mod day_06_tuning_trouble;
mod day_07_no_space_left_on_device;
mod day_07_no_space_left_on_device_with_tree;
mod day_08_treetop_tree_house;
mod day_09_rope_bridge;
mod day_10_cathode_ray_tube;
mod day_11_monkey_in_the_middle;
mod day_12_hill_climbing_algorithm;
mod day_13_distress_signal;
mod day_14_regolith_reservoir;
mod day_15_beacon_exclusion_zone;
mod day_16_proboscidea_volcanium;
mod day_17_pyroclastic_flow;
mod day_18_boiling_boulder;
mod day_19_not_enough_minerals;
mod day_20_grove_positioning_system;
mod day_21_monkey_math;
mod day_22_monkey_map;
mod day_23_unstable_diffusion;
mod day_24_blizzard_basin;
mod day_25_full_of_hot_air;
mod dijkstra;
mod sample_tests;

fn main() -> ExitCode {
    let args: Vec<_> = env::args().collect();
    let days = [
        day_01_calorie_counting::run,
        day_02_rock_paper_scissors::run,
        day_03_rucksack_reorganization::run,
        day_04_camp_cleanup::run,
        day_05_supply_stacks::run,
        day_06_tuning_trouble::run,
        day_07_no_space_left_on_device::run,
        day_08_treetop_tree_house::run,
        day_09_rope_bridge::run,
        day_10_cathode_ray_tube::run,
        day_11_monkey_in_the_middle::run,
        day_12_hill_climbing_algorithm::run,
        day_13_distress_signal::run,
        day_14_regolith_reservoir::run,
        day_15_beacon_exclusion_zone::run,
        day_16_proboscidea_volcanium::run,
        day_17_pyroclastic_flow::run,
        day_18_boiling_boulder::run,
        day_19_not_enough_minerals::run,
        day_20_grove_positioning_system::run,
        day_21_monkey_math::run,
        day_22_monkey_map::run,
        day_23_unstable_diffusion::run,
        day_24_blizzard_basin::run,
        day_25_full_of_hot_air::run,
    ];

    let run_single_day = |day_num: usize| {
        let instant = time::Instant::now();
        let filename = format!("inputs/day{day_num:02}.txt");
        match fs::read_to_string(&filename) {
            Ok(input) => {
                let output = days[day_num - 1](&input);
                let time_annotation = format_time_annotation(instant.elapsed());
                Ok(format!("Day {day_num}{time_annotation}: {output}"))
            }
            Err(err) => Err(format!("Error reading {filename}: {err}")),
        }
    };
    let print_day_result = |result: &_| match result {
        Ok(output) => println!("{output}"),
        Err(err_output) => eprintln!("{err_output}"),
    };

    match args.len() {
        1 => {
            let results: Vec<_> = (1..=days.len())
                .into_par_iter()
                .map(|day_num| run_single_day(day_num))
                .collect();

            for result in results {
                print_day_result(&result);
                if result.is_err() {
                    return ExitCode::FAILURE;
                }
            }
        }
        2 => {
            let Ok(day_num) = args[1].parse::<usize>() else {
                eprintln!("Invalid day number");
                return ExitCode::FAILURE;
            };
            if day_num < 1 || day_num > days.len() {
                eprintln!("Day number out of range");
                return ExitCode::FAILURE;
            }

            let result = run_single_day(day_num);
            print_day_result(&result);
            if result.is_err() {
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

fn format_time_annotation(elapsed: time::Duration) -> String {
    if elapsed.as_millis() < 1 {
        "".to_string()
    } else {
        format!(" ({elapsed:.0?})")
    }
}
