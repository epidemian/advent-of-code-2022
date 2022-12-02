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
            let own_choice = get_shape_for_result(opponent_choice, line.as_bytes()[2]);
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

fn single_round_score(opponent_choice: Shape, own_choice: Shape) -> u32 {
    let shape_score = match own_choice {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    let outcome_score = match (opponent_choice, own_choice) {
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
        (a, b) if a == b => 3,
        _ => 0,
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

fn get_shape_for_result(opponent_choice: Shape, result: u8) -> Shape {
    match result {
        b'X' => match opponent_choice {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        },
        b'Y' => opponent_choice,
        b'Z' => match opponent_choice {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        },
        _ => unreachable!(),
    }
}
