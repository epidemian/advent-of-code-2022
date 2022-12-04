use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day1.txt").unwrap();
    let mut sorted_calories_by_elf: Vec<u32> = input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect();
    sorted_calories_by_elf.sort();

    let max_calories_carried_by_single_elf = sorted_calories_by_elf.last().unwrap();
    println!("{}", max_calories_carried_by_single_elf);

    let max_calories_carried_by_3_elves = sorted_calories_by_elf.iter().rev().take(3).sum::<u32>();
    println!("{}", max_calories_carried_by_3_elves);
}
