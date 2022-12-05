use std::{
    fs::File,
    io::{prelude::*, BufReader},
};
fn main() {
    part_2()
}

fn part_1() {
    let file = File::open("./inputs/day4.txt").expect("no such file");
    let buf = BufReader::new(file);

    let mut count = 0;

    for line_res in buf.lines() {
        let line = line_res.expect("Could not parse line");
        let assignments: Vec<&str> = line.split(",").collect();
        let left = assignments[0];
        let right = assignments[1];

        let left_parts: Vec<&str> = left.split("-").collect();
        let left_low = left_parts[0].parse::<i32>().unwrap();
        let left_high = left_parts[1].parse::<i32>().unwrap();

        let right_parts: Vec<&str> = right.split("-").collect();
        let right_low = right_parts[0].parse::<i32>().unwrap();
        let right_high = right_parts[1].parse::<i32>().unwrap();

        if left_low <= right_low && left_high >= right_high {
            //left fully contains right
            count += 1;
        } else if right_low <= left_low && right_high >= left_high {
            // right fully contains left
            count += 1;
        }
    }

    println!("Count is {}", count);
}

fn part_2() {
    let file = File::open("./inputs/day4.txt").expect("no such file");
    let buf = BufReader::new(file);

    let mut count = 0;

    for line_res in buf.lines() {
        let line = line_res.expect("Could not parse line");
        let assignments: Vec<&str> = line.split(",").collect();
        let left = assignments[0];
        let right = assignments[1];

        let left_parts: Vec<&str> = left.split("-").collect();
        let left_low = left_parts[0].parse::<i32>().unwrap();
        let left_high = left_parts[1].parse::<i32>().unwrap();

        let right_parts: Vec<&str> = right.split("-").collect();
        let right_low = right_parts[0].parse::<i32>().unwrap();
        let right_high = right_parts[1].parse::<i32>().unwrap();

        // remember, rust ranges are inclusive low, exclusive high!
        let mut left_range = left_low..left_high + 1;
        let right_range = right_low..right_high + 1;

        if left_range.any(|item| right_range.contains(&item)) {
            count += 1;
        }
    }

    println!("Count is {}", count);
}
