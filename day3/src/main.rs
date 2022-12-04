use std::fs;
use itertools::Itertools;

const LC_PRIORITY_BASE : u32 = 'a' as u32;

fn priority(c : char) -> i32 {
    if c.is_lowercase() {
        (c as u32 - ('a' as u32) + 1) as i32
    } else if c.is_uppercase() {
        (c as u32 - ('A' as u32) + 27) as i32
    } else {
        panic!("Unknown item {}", c)
    }
}

fn main() {
    let text = fs::read_to_string("rucksacks.txt").unwrap();
    let total : i32 = text.lines().map(|line| {
        // Split rucksack into compartments.
        let (a, b) = line.split_at(line.len() / 2);
        let in_both = a.chars().filter(|c| { b.contains(*c) } )
            .unique().collect::<String>();
        // Find items in both compartments, calculate priority.
        let prio : i32 = in_both.chars().map(priority).sum();
        println!("({}, {}) both={} prio={}", a, b, in_both, prio);
        prio
    }).sum();
    println!("total={}", total);
}
