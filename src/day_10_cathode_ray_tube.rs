pub fn run(input: &str) -> String {
    let mut x = 1;
    let mut x_values_by_cycle = vec![];

    for ins in input.lines().map(parse_instruction) {
        match ins {
            Noop => x_values_by_cycle.push(x),
            Addx(arg) => {
                x_values_by_cycle.push(x);
                x_values_by_cycle.push(x);
                x += arg
            }
        }
    }

    let signal_strengths_sum: i32 = [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|cycle| cycle as i32 * x_values_by_cycle[cycle - 1])
        .sum();

    let mut crt_image = String::new();
    for y in 0..6 {
        for x in 0..40 {
            let sprite_x = x_values_by_cycle[y * 40 + x];
            let in_sprite = (sprite_x - x as i32).abs() <= 1;
            let pixel = if in_sprite { '#' } else { '.' };
            crt_image.push(pixel);
        }
        crt_image.push('\n');
    }
    let crt_image = crt_image.trim_end();

    format!("{signal_strengths_sum}\n{crt_image}")
}

enum Instruction {
    Noop,
    Addx(i32),
}

use Instruction::*;

fn parse_instruction(s: &str) -> Instruction {
    let words: Vec<_> = s.split(' ').collect();
    match words[0] {
        "noop" => Noop,
        "addx" => Addx(words[1].parse().expect("addx arg must be a valid number")),
        ins => unreachable!("invalid instruction {ins}"),
    }
}
