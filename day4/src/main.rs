use itertools::Itertools;
use std::fs;

fn range_contains_range(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

fn main() {
    let text = fs::read_to_string("pairs.txt").unwrap();
    let total: i32 = text
        .lines()
        .map(|line| {
            // Split each line into pairs of ranges.
            let (a, b): ((i32, i32), (i32, i32)) = line
                .split(",")
                .map(|s| {
                    s.split("-")
                        .map(|n| {
                            // Split the range into two numbers and parse.
                            n.parse::<i32>().unwrap()
                        })
                        .collect_tuple()
                        .unwrap() // Make the range into a tuple of integers.
                })
                .collect_tuple()
                .unwrap(); // Make the pair into a tuple of ranges.
            println!("{:?} {:?}", a, b);
            if range_contains_range(a, b) || range_contains_range(b, a) {
                1
            } else {
                0
            }
        })
        .sum();
    println!("total={}", total);
}
