use std::{
    fs::File,
    io::{prelude::*, BufReader},
};
fn main() {
    part_2()
}

fn part_1() {
    // This is part of my puzzle input - just didn't feel like parsing it :-D
    let mut stacks: [Vec<char>; 9] = [
        "WBDNCFJ".chars().collect(),
        "PZVQLST".chars().collect(),
        "PZBGJT".chars().collect(),
        "DTLJZBHC".chars().collect(),
        "GVBJS".chars().collect(),
        "PSQ".chars().collect(),
        "BVDFLMPN".chars().collect(),
        "PSMFBDLR".chars().collect(),
        "VDTR".chars().collect(),
    ];

    let file = File::open("./inputs/day5.txt").expect("no such file");
    let buf = BufReader::new(file);

    for line_res in buf.lines() {
        let line = line_res.expect("Could not parse line");
        let words: Vec<&str> = line.split(" ").collect();

        let movenum = words[1].parse::<i32>().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;
        for _ in 0..movenum {
            let popped = stacks[from].pop().unwrap();
            stacks[to].push(popped);
        }
    }

    let output: Vec<String> = stacks
        .iter()
        .map(|stack| stack.last().unwrap().to_string())
        .collect();
    println!("output is {}", output.join(""));
}

fn part_2() {
    // This is part of my puzzle input - just didn't feel like parsing it :-D
    let mut stacks: [Vec<char>; 9] = [
        "WBDNCFJ".chars().collect(),
        "PZVQLST".chars().collect(),
        "PZBGJT".chars().collect(),
        "DTLJZBHC".chars().collect(),
        "GVBJS".chars().collect(),
        "PSQ".chars().collect(),
        "BVDFLMPN".chars().collect(),
        "PSMFBDLR".chars().collect(),
        "VDTR".chars().collect(),
    ];

    let file = File::open("./inputs/day5.txt").expect("no such file");
    let buf = BufReader::new(file);

    for line_res in buf.lines() {
        let line = line_res.expect("Could not parse line");

        let words: Vec<&str> = line.split(" ").collect();

        let movenum = words[1].parse::<i32>().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;

        let mut temp_stack: Vec<char> = vec![];
        for _ in 0..movenum {
            temp_stack.push(stacks[from].pop().unwrap());
        }
        for _ in 0..movenum {
            stacks[to].push(temp_stack.pop().unwrap());
        }
        assert!(temp_stack.len() == 0);
    }

    let output: Vec<String> = stacks
        .iter()
        .map(|stack| stack.last().unwrap().to_string())
        .collect();
    println!("output is {}", output.join(""));
}
