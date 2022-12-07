pub fn run(input: &str) {
    let lines: Vec<_> = input.lines().collect();
    let shared_items_priority_sum: u32 = lines
        .iter()
        .map(|line| {
            let rucksack_items = line.as_bytes();
            let (first_compartment, second_compartment) =
                rucksack_items.split_at(rucksack_items.len() / 2);
            let shared_item_type = *first_compartment
                .iter()
                .find(|item_type| second_compartment.contains(item_type))
                .expect("there should be a shared item type between compartments");
            priority_for_item_type(shared_item_type) as u32
        })
        .sum();
    println!("{}", shared_items_priority_sum);

    let group_badges_priority_sum: u32 = lines
        .chunks(3)
        .map(|elf_group| {
            let elf_1_rucksack = elf_group[0].as_bytes();
            let elf_2_rucksack = elf_group[1].as_bytes();
            let elf_3_rucksack = elf_group[2].as_bytes();
            let badge = *elf_1_rucksack
                .iter()
                .find(|item_type| {
                    elf_2_rucksack.contains(item_type) && elf_3_rucksack.contains(item_type)
                })
                .expect("there should be a shared item type between the 3 elves");
            priority_for_item_type(badge) as u32
        })
        .sum();
    println!("{}", group_badges_priority_sum);
}

fn priority_for_item_type(item_type: u8) -> u8 {
    match item_type {
        b'a'..=b'z' => item_type - b'a' + 1,
        b'A'..=b'Z' => item_type - b'A' + 27,
        _ => unreachable!("invalid item type {}", item_type as char),
    }
}
