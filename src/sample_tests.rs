// This file was auto-generated by build.rs

#[cfg(test)]
mod tests {
    use crate::*;

    #[test] fn day_01_calorie_counting_sample() { run_sample_test(day_01_calorie_counting::run, "./samples/01.txt") }
    #[test] fn day_02_rock_paper_scissors_sample() { run_sample_test(day_02_rock_paper_scissors::run, "./samples/02.txt") }
    #[test] fn day_03_rucksack_reorganization_sample() { run_sample_test(day_03_rucksack_reorganization::run, "./samples/03.txt") }
    #[test] fn day_04_camp_cleanup_sample() { run_sample_test(day_04_camp_cleanup::run, "./samples/04.txt") }
    #[test] fn day_05_supply_stacks_sample() { run_sample_test(day_05_supply_stacks::run, "./samples/05.txt") }
    #[test] fn day_06_tuning_trouble_sample() { run_sample_test(day_06_tuning_trouble::run, "./samples/06.txt") }
    #[test] fn day_07_no_space_left_on_device_sample() { run_sample_test(day_07_no_space_left_on_device::run, "./samples/07.txt") }
    #[test] fn day_08_treetop_tree_house_sample() { run_sample_test(day_08_treetop_tree_house::run, "./samples/08.txt") }
    #[test] fn day_09_rope_bridge_large() { run_sample_test(day_09_rope_bridge::run, "./samples/09_large.txt") }
    #[test] fn day_09_rope_bridge_small() { run_sample_test(day_09_rope_bridge::run, "./samples/09_small.txt") }
    #[test] fn day_10_cathode_ray_tube_sample() { run_sample_test(day_10_cathode_ray_tube::run, "./samples/10.txt") }
    #[test] fn day_11_monkey_in_the_middle_sample() { run_sample_test(day_11_monkey_in_the_middle::run, "./samples/11.txt") }
    #[test] fn day_12_hill_climbing_algorithm_sample() { run_sample_test(day_12_hill_climbing_algorithm::run, "./samples/12.txt") }
    #[test] fn day_13_distress_signal_sample() { run_sample_test(day_13_distress_signal::run, "./samples/13.txt") }
    #[test] fn day_14_regolith_reservoir_sample() { run_sample_test(day_14_regolith_reservoir::run, "./samples/14.txt") }
    #[test] fn day_15_beacon_exclusion_zone_sample() { run_sample_test(day_15_beacon_exclusion_zone::run, "./samples/15.txt") }
    #[test] fn day_16_proboscidea_volcanium_sample() { run_sample_test(day_16_proboscidea_volcanium::run, "./samples/16.txt") }
    #[test] fn day_17_pyroclastic_flow_sample() { run_sample_test(day_17_pyroclastic_flow::run, "./samples/17.txt") }
    #[test] fn day_18_boiling_boulder_sample() { run_sample_test(day_18_boiling_boulder::run, "./samples/18.txt") }
    #[test] fn day_19_not_enough_minerals_sample() { run_sample_test(day_19_not_enough_minerals::run, "./samples/19.txt") }
    #[test] fn day_20_grove_positioning_system_sample() { run_sample_test(day_20_grove_positioning_system::run, "./samples/20.txt") }
    #[test] fn day_21_monkey_math_sample() { run_sample_test(day_21_monkey_math::run, "./samples/21.txt") }
    #[test] fn day_22_monkey_map_sample() { run_sample_test(day_22_monkey_map::run, "./samples/22.txt") }
    #[test] fn day_23_unstable_diffusion_sample() { run_sample_test(day_23_unstable_diffusion::run, "./samples/23.txt") }
    #[test] fn day_24_blizzard_basin_sample() { run_sample_test(day_24_blizzard_basin::run, "./samples/24.txt") }
    #[test] fn day_25_full_of_hot_air_sample() { run_sample_test(day_25_full_of_hot_air::run, "./samples/25.txt") }

    fn run_sample_test(day_fn: fn(&str) -> String, path: &str) {
        let content = std::fs::read_to_string(path).expect("error reading sample file");
        let (input, expected_output) = content
            .split_once("\n~~~\n")
            .expect("expected '~~~' on sample file");
        let output = day_fn(input);
        assert_eq!(output, expected_output.trim());
    }
}