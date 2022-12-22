use std::collections::HashSet;
use std::iter::repeat;

pub fn run(input: &str) -> String {
    format!("{} {}", run_rope_sym(2, input), run_rope_sym(10, input))
}

fn run_rope_sym(rope_length: usize, input: &str) -> usize {
    let mut rope = vec![(0, 0); rope_length];
    let mut visited_positions: HashSet<(i32, i32)> = [(0, 0)].into_iter().collect();

    for direction in parse_step_movements(input) {
        let (head_x, head_y) = &mut rope[0];
        match direction {
            "L" => *head_x -= 1,
            "R" => *head_x += 1,
            "U" => *head_y += 1,
            "D" => *head_y -= 1,
            _ => unreachable!("invalid direction {direction}"),
        }

        for i in 1..rope.len() {
            let (prev_knot_x, prev_knot_y) = rope[i - 1];
            let (knot_x, knot_y) = &mut rope[i];

            let dx: i32 = prev_knot_x - *knot_x;
            let dy: i32 = prev_knot_y - *knot_y;

            let knots_touching = dx.abs() <= 1 && dy.abs() <= 1;
            if !knots_touching {
                *knot_x += dx.clamp(-1, 1);
                *knot_y += dy.clamp(-1, 1);
            }
        }

        let tail = rope[rope.len() - 1];
        visited_positions.insert(tail);
    }
    visited_positions.len()
}

// Flattens moves instructions like "L 3" into 3 "L"s.
fn parse_step_movements(input: &str) -> impl Iterator<Item = &str> {
    input.lines().flat_map(|line| {
        let (direction, step_count) = line.split_once(' ').expect("invalid line");
        let step_count: usize = step_count.parse().expect("invalid number");
        repeat(direction).take(step_count)
    })
}
