use std::{
    fs::File,
    io::{prelude::*, BufReader},
};
fn main() {
    part_2()
}

fn part_1() {
    let file = File::open("./inputs/day3.txt").expect("no such file");
    let buf = BufReader::new(file);

    let mut total_priority = 0;

    for line_res in buf.lines() {
        let line = line_res.expect("Could not parse line");

        let sack_1 = &line[0..line.len() / 2];
        let sack_2 = &line[sack_1.len()..];

        let mut found_in_both: Vec<char> = vec![];
        for item in sack_1.chars() {
            for item2 in sack_2.chars() {
                if item == item2 && !found_in_both.contains(&item) {
                    found_in_both.push(item.clone())
                }
            }
        }

        for c in found_in_both {
            total_priority += get_priority(&c)
        }
    }

    println!("total priority is {}", total_priority);
}

fn part_2() {
    let file = File::open("./inputs/day3.txt").expect("no such file");
    let buf = BufReader::new(file);

    let mut total_priority = 0;

    let lines: Vec<String> = buf
        .lines()
        .into_iter()
        .map(|line_res| line_res.expect(""))
        .collect();

    for n in 0..lines.len() / 3 {
        let elf1 = lines[n * 3].to_string();
        let elf2 = lines[n * 3 + 1].to_string();
        let elf3 = lines[n * 3 + 2].to_string();

        match find_badge(elf1, elf2, elf3) {
            Some(b) => total_priority += get_priority(&b),
            None => panic!("badge not found"),
        };
    }

    println!("total priority is {}", total_priority);
}

fn find_badge(elf1: String, elf2: String, elf3: String) -> Option<char> {
    for item1 in elf1.chars() {
        for item2 in elf2.chars() {
            for item3 in elf3.chars() {
                if item1 == item2 && item2 == item3 {
                    return Some(item1);
                }
            }
        }
    }
    return None;
}

// Get priority based on codepoint offset
fn get_priority(ch: &char) -> u32 {
    let uppers: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    if uppers.contains(ch) {
        return (*ch as u32) - 38;
    } else {
        return (*ch as u32) - 96;
    }
}
