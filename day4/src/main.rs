use itertools::Itertools;
use std::fs;

// Make some type aliases for readability.
type Range = (i32, i32);

type Pair = (Range, Range);

// Split the range into two numbers and parse, return a tuple of integers.
fn parse_range(s : &str) -> Range {
    s.split("-")
        .map(|n| {
            n.parse::<i32>().unwrap()
        })
        .collect_tuple()
        .unwrap() // Since Range is an alias for (i32, i32), this "just works".
}

// Parse each line into a pair of ranges.
fn parse_line(line : &str) -> Pair {
    line
        .split(",")
        .map(parse_range)
        .collect_tuple()
        .unwrap()   // Since Point is an alias for (Range, Range), this "just works".
}

fn range_contains_range(a: Range, b: Range) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

fn main() {
    let text = fs::read_to_string("pairs.txt").unwrap();
    let total: i32 = text
        .lines()
        .map(|line| {
            // Split each line into pairs of ranges.
            // We can destructure Pair, because it's really just a tuple.
            let (a, b) = parse_line(line);
            println!("{:?} {:?}", a, b);
            // Return 1 (to increment the total) if there is an overlap.  Zero if not.
            if range_contains_range(a, b) || range_contains_range(b, a) {
                1
            } else {
                0
            }
        })
        .sum(); // Add 'em all up.
    println!("total={}", total);
}
