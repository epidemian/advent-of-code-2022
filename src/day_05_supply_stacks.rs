pub fn run(input: &str) -> String {
    let (initial_stacks_section, crane_moves_section) = input
        .split_once("\n\n")
        .expect("input should have two sections separated by double newlines");
    let initial_stacks = parse_stacks(initial_stacks_section);
    let crane_moves = parse_moves(crane_moves_section);
    format!(
        "{} {}",
        exec_moves(&initial_stacks, &crane_moves, move_as_crate_mover_9000),
        exec_moves(&initial_stacks, &crane_moves, move_as_crate_mover_9001)
    )
}

type Stack = Vec<char>;
struct Move(usize, usize, usize);
type MoveFn = fn(&Move, &mut [Stack]) -> ();

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

fn exec_moves(initial_stacks: &[Stack], crane_moves: &[Move], move_fn: MoveFn) -> String {
    let mut stacks = initial_stacks.to_vec();
    for crane_move in crane_moves.iter() {
        move_fn(crane_move, &mut stacks)
    }
    get_top_crate_letters(&stacks)
}

fn move_as_crate_mover_9000(&Move(crate_count, from, to): &Move, stacks: &mut [Stack]) {
    for _ in 0..crate_count {
        let top_crate = stacks[from].pop().expect("stack should have a crate");
        stacks[to].push(top_crate);
    }
}

fn move_as_crate_mover_9001(&Move(crate_count, from, to): &Move, stacks: &mut [Stack]) {
    let bottom_crate_index = stacks[from].len() - crate_count;
    let moved_crates = stacks[from].split_off(bottom_crate_index);
    stacks[to].extend(moved_crates);
}

fn get_top_crate_letters(stacks: &[Stack]) -> String {
    stacks.iter().filter_map(|stack| stack.last()).collect()
}
