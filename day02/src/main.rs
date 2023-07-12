/*
 * A, X = Rock      1 point
 * B, Y = Paper     2 point
 * C, Z = Scissors  3 point
 *
 * loss = 0 point
 * draw = 3 point
 * wins = 6 point
 *
  * */

use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("File could not be read");
    run(&contents);
    run2(&contents);
}

fn run(input: &String) {
    let mut score: u32 = 0;

    for round in input.trim().split("\n") {
        let play: String = round.split(" ").collect();
        let opp_move = play.chars().nth(0).unwrap();
        let our_move = play.chars().nth(1).unwrap();
        let our_score = match our_move {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => 0
        };

        let result = match opp_move {
            'A' => match our_move {
                'X' => 3,
                'Y' => 6,
                'Z' => 0,
                _ => 0
            },
            'B' => match our_move {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                _ => 0
            },
            'C' => match our_move {
                'X' => 6,
                'Y' => 0,
                'Z' => 3,
                _ => 0
            },
            _ => 0
        };

        score += our_score + result;
    }

    println!("{score}");
}

fn run2(input: &String) {
    let mut score: u32 = 0;

    for round in input.trim().split("\n") {
        let play: String = round.split(" ").collect();
        let opp_move = play.chars().nth(0).unwrap();
        let game_res = play.chars().nth(1).unwrap();
        let our_score = match game_res {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => 0
        };

        let result = match opp_move {
            'A' => match game_res {
                'X' => 3,
                'Y' => 1,
                'Z' => 2,
                _ => 0
            },
            'B' => match game_res {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => 0
            },
            'C' => match game_res {
                'X' => 2,
                'Y' => 3,
                'Z' => 1,
                _ => 0
            },
            _ => 0
        };

        score += our_score + result;
    }

    println!("{score}");
}
