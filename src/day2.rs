use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day2.txt").unwrap();
    let ans1: u32 = input
        .lines()
        .map(|line| {
            let opponent_choice = parse_shape(line.as_bytes()[0]);
            let own_choice = parse_shape(line.as_bytes()[2]);
            single_round_score(opponent_choice, own_choice)
        })
        .sum();
    println!("{}", ans1);

    let ans2: u32 = input
        .lines()
        .map(|line| {
            let opponent_choice = parse_shape(line.as_bytes()[0]);
            let result = match line.as_bytes()[2] {
                b'X' => Lose,
                b'Y' => Draw,
                b'Z' => Win,
                _ => unreachable!(),
            };
            let own_choice = get_shape_for_result(opponent_choice, result);
            single_round_score(opponent_choice, own_choice)
        })
        .sum();
    println!("{}", ans2);
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
use Shape::*;

enum RoundResult {
    Win,
    Draw,
    Lose,
}
use RoundResult::*;

fn single_round_score(opponent_choice: Shape, own_choice: Shape) -> u32 {
    let shape_score = match own_choice {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    let outcome_score = if own_choice == get_shape_for_result(opponent_choice, Win) {
        6
    } else if own_choice == opponent_choice {
        3
    } else {
        0
    };

    shape_score + outcome_score
}

fn parse_shape(char: u8) -> Shape {
    match char {
        b'A' | b'X' => Rock,
        b'B' | b'Y' => Paper,
        b'C' | b'Z' => Scissors,
        _ => unreachable!(),
    }
}

fn get_shape_for_result(opponent_choice: Shape, result: RoundResult) -> Shape {
    match result {
        Lose => match opponent_choice {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        },
        Draw => opponent_choice,
        Win => match opponent_choice {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        },
    }
}
