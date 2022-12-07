
fn is_marker(buf: &str) -> bool {
    if buf.len() != 4 { false }
    else {
        let mut encountered: Vec<char> = Vec::new();
        for c in buf.chars() {
            if encountered.contains(&c) { return false; }
            encountered.push(c);
        }
        true
    }
}

fn find_start_index(buf: &str) -> i32 {
    // Find the first set of four consecutive chars that are all unique.
    for i in 0..buf.len()-4 {
        let word = &buf[i..i+4];
        if is_marker(word) {
            return i as i32 + 4
        }
    }
    -1
}

fn example(buf: &str) {
    let index = find_start_index(buf);
    println!("'{}' first marker after character {}", buf, index)
}

fn main() {
    example("bvwbjplbgvbhsrlpgdmjqwftvncz");
    example("nppdvjthqldpwncqszvftbrmjlhg");
    example("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
    example("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_is_marker() {
        assert_eq!(false, is_marker("aaaa"));
        assert_eq!(true, is_marker("abcd"));
        assert_eq!(false, is_marker("aaa"))
    }

    #[test]
    fn test_find_start_index() {
        assert_eq!(4, find_start_index("abcddddd"));
    }
}