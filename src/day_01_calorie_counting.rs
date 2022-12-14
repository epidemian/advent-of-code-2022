pub fn run(input: &str) -> String {
    let mut sorted_calories_by_elf: Vec<u32> = input
        .split("\n\n")
        .map(|elf_items| {
            elf_items
                .lines()
                .map(|line| {
                    line.parse::<u32>()
                        .expect("each line should have a valid number of calories")
                })
                .sum()
        })
        .collect();
    sorted_calories_by_elf.sort();

    let max_calories_on_single_elf = *sorted_calories_by_elf.last().unwrap();
    let max_calories_on_3_elves = sorted_calories_by_elf.iter().rev().take(3).sum::<u32>();

    format!("{max_calories_on_single_elf} {max_calories_on_3_elves}")
}
