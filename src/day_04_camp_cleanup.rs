pub fn run(input: &str) -> String {
    let mut fully_overlapping_pairs = 0;
    let mut overlapping_pairs = 0;

    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split(&['-', ','])
            .map(|s| s.parse().expect("section IDs should be numbers"))
            .collect();
        let [elf_1_start, elf_1_end, elf_2_start, elf_2_end] = numbers[..] else {
            panic!("invalid line format {}", line);
        };

        if (elf_1_start >= elf_2_start && elf_1_end <= elf_2_end)
            || (elf_2_start >= elf_1_start && elf_2_end <= elf_1_end)
        {
            fully_overlapping_pairs += 1;
        }

        if elf_1_end >= elf_2_start && elf_1_start <= elf_2_end {
            overlapping_pairs += 1;
        }
    }

    format!("{fully_overlapping_pairs} {overlapping_pairs}")
}
