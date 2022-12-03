use std::fs;
use std::str::Split;

use crate::Hand::{Paper, Rock, Scissors};

#[derive(PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

trait Beats {
    fn beats(&self) -> Self;
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        match *self {
            Paper => Rock,
            Rock => Scissors,
            Scissors => Paper,
        }
    }
}

trait Loses {
    fn loses(&self) -> Self;
}

impl Loses for Hand {
    fn loses(&self) -> Self {
        match *self {
            Paper => Scissors,
            Rock => Paper,
            Scissors => Rock,
        }
    }
}

trait Points {
    fn points(&self) -> i32;
}

impl Points for Hand {
    fn points(&self) -> i32 {
        match *self {
            Rock => { 1 }
            Paper => { 2 }
            Scissors => { 3 }
        }
    }
}

struct Game {
    my_move: Hand,
    opponent_move: Hand,
}

fn parse_move(input: char) -> Hand {
    match input {
        'A' | 'X' => Rock,
        'B' | 'Y' => Paper,
        'C' | 'Z' => Scissors,
        _ => { Rock }
    }
}

fn calculate_move(input: char, opponent_move: Hand) -> Hand {
    match input {
        'X' => opponent_move.beats(),
        'Y' => opponent_move,
        'Z' => opponent_move.loses(),
        _ => { Rock }
    }
}

fn get_points(game: Game) -> i32 {
    if game.my_move.beats() == game.opponent_move {
        6 + game.my_move.points()
    } else if game.opponent_move.beats() == game.my_move {
        game.my_move.points()
    } else {
        3 + game.my_move.points()
    }
}

fn part_one(games: Split<char>) {
    let mut score: i32 = 0;
    for line in games {
        let char_vec: Vec<char> = line.chars().collect();
        let game = Game { my_move: parse_move(char_vec[2]), opponent_move: parse_move(char_vec[0]) };
        score += get_points(game);
    }

    println!("Result for part one: {:?}", score);
}

fn part_two(games: Split<char>) {
    let mut score: i32 = 0;
    for line in games {
        let char_vec: Vec<char> = line.chars().collect();
        let opponent_move = parse_move(char_vec[0]);
        let game = Game {my_move: calculate_move(char_vec[2], opponent_move), opponent_move };
        score += get_points(game);
    }

    println!("Result for part two: {:?}", score);
}

fn main() {
    let file_path = "src/input.txt";
    let input = fs::read_to_string(file_path).expect("File not found");

    part_one(input.split('\n'));
    part_two(input.split('\n'));
}