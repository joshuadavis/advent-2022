extern crate core;

use std::fs;
use crate::Choice::*;
use crate::Outcome::*;

#[derive(Debug, Copy, Clone)]
enum Choice {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    WIN = 6,
    LOSE = 0,
    DRAW = 3
}

// Computes the outcome.
fn outcome(a: Choice, b: Choice) -> Outcome {
    match a {
        ROCK => {
            match b {
                ROCK => { DRAW }
                PAPER => { LOSE }
                SCISSORS => { WIN }
            }
        }
        PAPER => {
            match b {
                ROCK => { WIN }
                PAPER => { DRAW }
                SCISSORS => { LOSE }
            }
        }
        SCISSORS => {
            match b {
                ROCK => { LOSE }
                PAPER => { WIN }
                SCISSORS => { DRAW }
            }
        }
    }
}

// Computes the score based on the outcome.
fn outcome_score(a: Choice, b: Choice) -> i32 {
    outcome(a, b) as i32
}

// Computes the total score for 'a' (in a vs b)
fn score(a: Choice, b:Choice) -> i32 {
    // Choice discriminator is the score for the choice.
    // Outcome discriminator is the score for the outcome.
    // In Rust, enum discriminators are i32 by default, so we just cast them to get the value.
    a as i32 + outcome(a , b) as i32
}

fn decode_opponent(s: &str) -> Choice {
    match s {
        "A" => { ROCK }
        "B" => { PAPER }
        "C" => { SCISSORS }
        _ => { panic!("Unknown choice: {}", s) }
    }
}

fn decode_player(s: &str) -> Choice {
    match s {
        "X" => { ROCK }
        "Y" => { PAPER }
        "Z" => { SCISSORS }
        _ => { panic!("Unknown choice: {}", s) }
    }
}

fn main() {
    let text = fs::read_to_string("strategy.txt").unwrap();
    let strategy : Vec<(Choice, Choice)> = text.split("\n").map( |line| {
        let words : Vec<&str> = line.split(" ").collect();
        (decode_opponent(words[0]), decode_player(words[1]))
    } ).collect();

    let mut total : i32 = 0;
    for l in strategy {
        let outcome = outcome(l.1, l.0);
        let score = score(l.1, l.0);
        total += score;
        println!("player={:?} opponent={:?} outcome={:?} score={} total={}", l.1, l.0, outcome,
                 score, total);
    }
}
