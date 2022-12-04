use std::fs;

fn main() {
    let mut elves : Vec<i32> = fs::read_to_string("elves.txt").unwrap()
        .split("\n\n")// Note: won't work on WinDoze, because.
        .map(|s| {
            // Parse each item quantity and add them up.
            s.split("\n").map( |e| { e.parse::<i32>().unwrap() }).sum()
        }).collect();
    elves.sort();
    for e in elves {
        println!("e={}", e);
    }
}
