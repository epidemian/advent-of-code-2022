pub fn run(input: &str) -> (u32, u32) {
    let mut fully_overlapping_pairs = 0;
    let mut overlapping_pairs = 0;

    for line in input.lines() {
        let (first_elf_range, second_elf_range) = line
            .split_once(",")
            .expect("line should be a comma-separated pair of ranges");
        let (elf_1_start, elf_1_end) = parse_range(first_elf_range);
        let (elf_2_start, elf_2_end) = parse_range(second_elf_range);

        if (elf_1_start >= elf_2_start && elf_1_end <= elf_2_end)
            || (elf_2_start >= elf_1_start && elf_2_end <= elf_1_end)
        {
            fully_overlapping_pairs += 1;
        }

        if elf_1_end >= elf_2_start && elf_1_start <= elf_2_end {
            overlapping_pairs += 1;
        }
    }

    (fully_overlapping_pairs, overlapping_pairs)
}

fn parse_range(s: &str) -> (u32, u32) {
    let (start, end) = s.split_once("-").expect("ranges should have a hyphen");
    (start.parse().unwrap(), end.parse().unwrap())
}
