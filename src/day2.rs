use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day2.txt").unwrap();
    let ans1: u32 = input
        .lines()
        .map(|line| {
            let opponent_choice = parse_shape(line.as_bytes()[0]);
            let own_choice = parse_shape(line.as_bytes()[2]);
            own_choice as u32 + get_round_outcome_score(opponent_choice, own_choice)
        })
        .sum();
    println!("{}", ans1);

    let ans2: u32 = input
        .lines()
        .map(|line| {
            let opponent_choice = parse_shape(line.as_bytes()[0]);
            let round_result = match line.as_bytes()[2] {
                b'X' => LOSE,
                b'Y' => DRAW,
                b'Z' => WIN,
                _ => unreachable!(),
            };
            let own_choice = *[Rock, Paper, Scissors]
                .iter()
                .find(|&&shape| get_round_outcome_score(opponent_choice, shape) == round_result)
                .unwrap();
            own_choice as u32 + round_result
        })
        .sum();
    println!("{}", ans2);
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
use Shape::*;

const LOSE: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

fn get_round_outcome_score(opponent_choice: Shape, own_choice: Shape) -> u32 {
    match (opponent_choice, own_choice) {
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => WIN,
        (a, b) if a == b => DRAW,
        _ => LOSE,
    }
}

fn parse_shape(char: u8) -> Shape {
    match char {
        b'A' | b'X' => Rock,
        b'B' | b'Y' => Paper,
        b'C' | b'Z' => Scissors,
        _ => unreachable!(),
    }
}
