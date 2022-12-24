use anyhow::{anyhow, Result};
use std::fs;

fn is_edge<T>(n: usize, v: &Vec<T>) -> bool {
    n <= 0 || n >= v.len()-1
}

fn is_taller_than_neighbors(lines: &Vec<Vec<u32>>, x: usize, y: usize, h: u32) -> bool {
    let (up, down, left, right) = (lines[y-1][x], lines[y+1][x], lines[y][x-1], lines[y][x+1]);
    up < h && down < h && left < h && right < h
}

fn main() -> Result<()> {
    let text = fs::read_to_string("tree-height.txt")?;
    let lines = text.lines().map( |line| {
        line.chars().map(|c| {
            // Map each char into a Result<u32>, by converting
            // the Option into a Result with 'ok_or' (which is really
            // "Some(x) or" in this case).
            c.to_digit(10).ok_or(anyhow!("bogus!"))
        })
        // Here we use the type system to return either the vector of
        // numbers OR, and error if any one of them was an error.
        // This propagates any error in the "map" closure.
        .collect::<Result<Vec<u32>>>()

    })
    // Same type system trick to propagate errors, this time bail if
    // it's an error with "?".
    .collect::<Result<Vec<Vec<u32>>>>()?;

    // Iterate through the matrix, looking at the neighbors.
    let visible = lines.iter().enumerate().map( | (y, row)| {
        row.iter().enumerate().map( | ( x, h ) | {
            if is_edge(x, row) || is_edge(y, &lines) {
                true // If the tree is on the edge, then it's visible.
            } else { // Otherwise, check the neighbors.
                is_taller_than_neighbors(&lines, x, y, *h)
            }
        }).collect::<Vec<bool>>()
    }).collect::<Vec<Vec<bool>>>();

    println!("visible={:?}", visible);

    let total = visible.iter()
        .map(|row| {
            let count = row.iter().filter(|v| { **v }).count();
            println!("row={:?} count={}", row, count);
            count
        })
        .sum::<usize>();
    println!("total={}", total);

    Ok(())
}
