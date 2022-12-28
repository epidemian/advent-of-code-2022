pub fn run(input: &str) -> String {
    let (initial_stacks_section, crane_moves_section) = input
        .split_once("\n\n")
        .expect("input should have two sections separated by double newlines");
    let initial_stacks = parse_stacks(initial_stacks_section);
    let crane_moves = parse_moves(crane_moves_section);

    let mut stacks = initial_stacks.clone();
    for &Move(crate_count, from, to) in crane_moves.iter() {
        for _ in 0..crate_count {
            let top_crate = stacks[from].pop().expect("stack should have a crate");
            stacks[to].push(top_crate);
        }
    }
    let part_1_ans = get_top_crate_letters(&stacks);

    let mut stacks = initial_stacks.clone();
    for &Move(crate_count, from, to) in crane_moves.iter() {
        let bottom_crate_index = stacks[from].len() - crate_count;
        let moved_crates = stacks[from].split_off(bottom_crate_index);
        stacks[to].extend(moved_crates);
    }
    let part_2_ans = get_top_crate_letters(&stacks);

    format!("{part_1_ans} {part_2_ans}")
}

type Stack = Vec<char>;
struct Move(usize, usize, usize);

fn parse_stacks(input: &str) -> Vec<Stack> {
    // Collect stacks in reverse order and ignore last line with stack names.
    let lines: Vec<_> = input.lines().rev().skip(1).map(str::as_bytes).collect();
    let mut stacks = vec![];
    for i in 0.. {
        let char_index = i * 4 + 1;
        if char_index >= lines[0].len() {
            break;
        }
        let stack: Stack = lines
            .iter()
            .map(|line| line[char_index] as char)
            .filter(|&ch| ch != ' ')
            .collect();
        stacks.push(stack);
    }
    stacks
}

fn parse_moves(input: &str) -> Vec<Move> {
    input.lines().map(parse_move).collect()
}

fn parse_move(line: &str) -> Move {
    let nums: Vec<usize> = line
        .split(' ')
        .filter_map(|word| word.parse().ok())
        .collect();
    let [crate_count, from, to] = nums[..] else {
        panic!("invalid line {line}")
    };
    Move(crate_count, from - 1, to - 1)
}

fn get_top_crate_letters(stacks: &[Stack]) -> String {
    stacks.iter().filter_map(|stack| stack.last()).collect()
}
