pub fn run(input: &str) -> (u32, u32) {
    let (initial_state_section, procedure_section) = input
        .split_once("\n\n")
        .expect("input should have two sections separated by double newlines");
    let mut stacks = parse_stacks(initial_state_section);
    for line in procedure_section.lines() {
        let (amount, from, to) = parse_move(line);
        for _ in 0..amount {
            let top_crate = stacks[from].pop().expect("stack should have a crate");
            stacks[to].push(top_crate);
        }
    }
    for stack in stacks {
        print!("{}", stack.last().expect("stack should have a crate"));
    }
    println!();

    let mut stacks = parse_stacks(initial_state_section);
    for line in procedure_section.lines() {
        let (amount, from, to) = parse_move(line);
        let first_crate_index = stacks[from].len() - amount;
        let mut crates: Vec<_> = stacks[from].drain(first_crate_index..).collect();
        stacks[to].append(&mut crates);
    }
    for stack in stacks {
        print!("{}", stack.last().expect("stack should have a crate"));
    }
    println!();

    (0, 0)
}

type Stack = Vec<char>;

fn parse_stacks(input: &str) -> Vec<Stack> {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    let mut stacks = vec![];
    for i in 0..9 {
        let char_index = i * 4 + 1;
        if char_index >= lines[0].len() {
            break;
        }
        let stack: Stack = lines
            .iter()
            .rev()
            .skip(1)
            .map(|line| line[char_index] as char)
            .filter(|&ch| ch != ' ')
            .collect();
        stacks.push(stack);
    }
    stacks
}

fn parse_move(line: &str) -> (usize, usize, usize) {
    let words: Vec<_> = line.split(' ').collect();
    let amount = words[1].parse().expect("expected a valid number");
    let from: usize = words[3].parse().expect("expected a valid number");
    let to: usize = words[5].parse().expect("expected a valid number");
    (amount, from - 1, to - 1)
}
