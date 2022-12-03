use std::fs;
use crate::Hand::{Paper, Rock, Scissors};

#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors
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
    opponent_move: Hand
}

fn parse_input(opponent_move: char, my_move: char) -> Game {
    let mut game: Game = Game { my_move: Hand::Rock, opponent_move: Hand::Rock };
    game.opponent_move = match opponent_move {
        'A' => Rock,
        'B' => Paper,
        'C' => Scissors,
        _ => { Rock }
    };
    game.my_move = match my_move {
        'X' => Rock,
        'Y' => Paper,
        'Z' => Scissors,
        _ => { Rock }
    };
    game
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

fn main() {
    let file_path = "src/input.txt";

    let input = fs::read_to_string(file_path).expect("File not found");
    let games = input.split('\n');

    let mut my_score: i32 = 0;
    for game in games {
        let char_vec: Vec<char> = game.chars().collect();
        let game = parse_input(char_vec[0], char_vec[2]);
        my_score += get_points(game);
    }

    println!("{:?}", my_score);
}