extern crate core;

use std::fs;
use crate::Choice::{PAPER, ROCK, SCISSORS};

#[derive(Debug, Copy, Clone)]
enum Choice {
    ROCK,
    PAPER,
    SCISSORS
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    WIN,
    LOSE,
    DRAW
}

// Computes the outcome.
fn outcome(a: Choice, b: Choice) -> Outcome {
    match a {
        Choice::ROCK => {
            match b {
                Choice::ROCK => { Outcome::DRAW }
                Choice::PAPER => { Outcome::LOSE }
                Choice::SCISSORS => { Outcome::WIN }
            }
        }
        Choice::PAPER => {
            match b {
                Choice::ROCK => { Outcome::WIN }
                Choice::PAPER => { Outcome::DRAW }
                Choice::SCISSORS => { Outcome::LOSE }
            }
        }
        Choice::SCISSORS => {
            match b {
                Choice::ROCK => { Outcome::LOSE }
                Choice::PAPER => { Outcome::WIN }
                Choice::SCISSORS => { Outcome::DRAW }
            }
        }
    }
}

// Computes the score based on the outcome.
fn outcome_score(a: Choice, b: Choice) -> i32 {
    match outcome(a, b) {
        Outcome::WIN => { 6 }
        Outcome::DRAW => { 3 }
        Outcome::LOSE => { 0 }
    }
}

// Computes the part of the score based on the choice.
fn choice_score(c: Choice) -> i32 {
    match c {
        Choice::ROCK => { 1 }
        Choice::PAPER => { 2 }
        Choice::SCISSORS => { 3 }
    }
}

// Computes the total score for 'a' (in a vs b)
fn score(a: Choice, b:Choice) -> i32 {
    choice_score(a) + outcome_score(a , b)
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
