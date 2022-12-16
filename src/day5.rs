pub fn run(input: &str) -> (u32, u32) {
    let (initial_state_section, procedure_section) = input
        .split_once("\n\n")
        .expect("input should have two sections separated by double newlines");
    let procedure_moves = parse_moves(procedure_section);
    let initial_stacks = parse_stacks(initial_state_section);

    let mut stacks = initial_stacks.clone();
    for &Move(amount, from, to) in procedure_moves.iter() {
        for _ in 0..amount {
            let top_crate = stacks[from].pop().expect("stack should have a crate");
            stacks[to].push(top_crate);
        }
    }
    for stack in stacks {
        print!("{}", stack.last().expect("stack should have a crate"));
    }
    println!();

    let mut stacks = initial_stacks.clone();
    for &Move(amount, from, to) in procedure_moves.iter() {
        let bottom_crate_index = stacks[from].len() - amount;
        let mut moved_crates: Vec<_> = stacks[from].drain(bottom_crate_index..).collect();
        stacks[to].append(&mut moved_crates);
    }
    for stack in stacks {
        print!("{}", stack.last().expect("stack should have a crate"));
    }
    println!();

    (0, 0)
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
    let [amount, from, to] = nums[..] else {
        panic!("invalid line {line}")
    };
    Move(amount, from - 1, to - 1)
}
