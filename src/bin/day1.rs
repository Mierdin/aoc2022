use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let file = File::open("./inputs/day1-1.txt").expect("no such file");
    let buf = BufReader::new(file);

    let mut highest_elf_calories = 0;
    let mut highest_elf_index = 0;
    let mut current_elf = 0;

    let mut current_elf_calories = 0;

    let mut calories_counts: Vec<i32> = vec![];

    for line_res in buf.lines() {
        let line = line_res.expect("Could not parse line");

        match line.len() {
            0 => {
                if current_elf_calories > highest_elf_calories {
                    highest_elf_calories = current_elf_calories;
                    highest_elf_index = current_elf;
                }

                calories_counts.push(current_elf_calories);

                current_elf += 1;
                current_elf_calories = 0;
            }
            _ => current_elf_calories += line.parse::<i32>().unwrap(),
        };
    }

    // Part 1 answer
    println!(
        "Highest elf is {}, carrying {} calories",
        highest_elf_index, highest_elf_calories
    );

    // Part 2 answer
    calories_counts.sort();
    let sum: i32 = calories_counts[calories_counts.len() - 3..].iter().sum();
    println!("Sum of top 3- {:?}", sum);
}
