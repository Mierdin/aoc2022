use std::collections::HashMap;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};
fn main() {
    // Part 1
    parts(4);

    // Part two just increases the window size - ezpz! :-P
    parts(14);
}

fn parts(window_size: usize) {
    let file = File::open("./inputs/day6.txt").expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();
    let chars: Vec<char> = lines[0].chars().collect();

    let mut current_pos = 0;
    while current_pos <= chars.len() - window_size {
        let current_window = &chars[current_pos..window_size + current_pos].to_vec();
        if !contains_duplicates(&current_window) {
            println!(
                "Found after processing {} characters",
                current_pos + window_size
            );
            // 1103 is too low
            break;
        }
        current_pos += 1;
    }
}

fn contains_duplicates(chars: &Vec<char>) -> bool {
    let mut map = HashMap::new();
    for char in chars {
        if map.contains_key(&char) {
            return true;
        }
        map.insert(char, "foo");
    }

    false
}
