use std::error::Error;
use std::fs;
use std::slice::Iter;
use simple_error::*;
use Line::*;

#[derive(Debug)]
enum Line {
    CdDotDot,
    Cd(String),
    Ls,
    Dir(String),
    File(i32, String)
}

fn parse_error(line : &str) -> Result<Line, SimpleError> {
    Err(SimpleError::new(format!("Can't parse {}", line)))
}

fn parse_line(line : &str) -> Result<Line, SimpleError> {
    let tokens : Vec<&str> = line.split(" ").collect();
    match tokens[0] {
        "$" => {
            if tokens.len() < 2 { parse_error(line) }
            else {
                match tokens[1] {
                    "cd" => {
                        if tokens.len() < 3 { parse_error(line) }
                        else {
                            match tokens[2] {
                                ".." => { Ok(CdDotDot) }
                                _ => Ok(Cd(String::from(tokens[2])))
                            }
                        }
                    }
                    "ls" => { Ok(Ls) }
                    _ => { parse_error(line) }
                }
            }
        }
        "dir" => {
            if tokens.len() < 2 { parse_error(line) }
            else { Ok(Dir(String::from(tokens[1]))) }
        }
        _ => {
            if tokens.len() < 2 { parse_error(line) }
            else {
                match tokens[0].parse::<i32>() {
                    Ok(sz) => {
                        Ok(File(sz, String::from(tokens[1])))
                    }
                    Err(e) => Err(SimpleError::from(e))
                }
            }
        }
    }
}

#[derive(Debug)]
struct Directory {
    path: String,
    size: i32
}

fn nextpath(path: &str, dir: &String) -> String {
    let next_path = if path.len() == 0 {
        String::from(dir)
    } else if path.ends_with("/") {
        String::from(path) + dir
    } else {
        String::from(path) + "/" + dir
    };
    next_path
}

fn total_size(path : &str, iter: &mut Iter<Line>, dirs: &mut Vec<Directory>) -> i32 {
    let mut size : i32 = 0;
    while let Some(line) = iter.next() {
        println!("path={}, line={:?}", path, line);
        match line {
            CdDotDot => {
                dirs.push(Directory { path: String::from(path), size});
                return size;
            }
            Cd(dir) => {
                size += total_size(nextpath(path, dir).as_str(), iter, dirs);   // Calculate size until "cd .."
            }
            File(sz, n) => {
                size += sz
            }
            _ => {}
        }
    }
    if !path.is_empty() {
        dirs.push(Directory { path: String::from(path), size});
    }
    println!("end of input, size={}", size);
    size
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("directory.txt")?;
    // Parse all the lines, filter out errors.
    let lines = text.lines().filter_map(|l| { parse_line(l).ok() }).collect::<Vec<Line>>();
    println!("Lines={:?}", lines);

    // Interpret the lines, store the entries in a map by path.
    let mut dirs : Vec<Directory> = Vec::new();
    total_size("", & mut lines.iter(), &mut dirs);

    println!("dirs={:?}", dirs);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn when_bad_line_then_err() {
        assert!(parse_line("").is_err());
        assert!(parse_line("$ ").is_err());
        assert!(parse_line("$ foo").is_err());
        assert!(parse_line("$ cd").is_err());
        assert!(parse_line("dir").is_err());
        assert!(parse_line("551").is_err());
    }

    #[test]
    fn when_good_line_then_ok() {
        assert!(parse_line("$ cd blah").is_ok());
        assert!(parse_line("dir x").is_ok());
        assert!(parse_line("551 foo.txt").is_ok());
    }

    #[test]
    fn string_pattern_matching() {
        let x = "/".ends_with("/");
        println!("x={}", x)
    }
}
