use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    part_2()
}

fn part_1() {
    let file = File::open("./inputs/day2.txt").expect("no such file");
    let buf = BufReader::new(file);

    let mut score = 0;

    for line_res in buf.lines() {
        let line = line_res.expect("Could not parse line");

        let components: Vec<&str> = line.split(" ").collect();
        let (me, them) = get_plays(components[1], components[0]);
        let me_val = get_me_value(&me);
        match get_round_outcome(&me, &them) {
            RoundOutcome::Win => {
                score += me_val + 6;
            }
            RoundOutcome::Draw => {
                score += me_val + 3;
            }
            RoundOutcome::Lose => {
                score += me_val;
            }
        }
    }

    println!("Final score is {}", score);
}

fn part_2() {
    let file = File::open("./inputs/day2.txt").expect("no such file");
    let buf = BufReader::new(file);

    let mut score = 0;

    for line_res in buf.lines() {
        let line = line_res.expect("Could not parse line");

        let components: Vec<&str> = line.split(" ").collect();
        // let (me, them) = get_plays(components[1], components[0]);

        let desired_outcome = get_outcome_from_str(components[1]);

        let play = get_plays_part_2(components[0], &desired_outcome);
        let play_val = get_me_value(&play);

        match desired_outcome {
            RoundOutcome::Win => {
                score += play_val + 6;
            }
            RoundOutcome::Draw => {
                score += play_val + 3;
            }
            RoundOutcome::Lose => {
                score += play_val;
            }
        }
    }

    println!("Final score is {}", score);
    // 9583 is too low
}

#[derive(Debug)]
pub enum RoundOutcome {
    Win,
    Lose,
    Draw,
}
#[derive(Debug)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

// Used for part 1, where we thought the second column was what I should play
fn get_plays(me: &str, them: &str) -> (Play, Play) {
    let my_play = match me {
        "X" => Play::Rock,
        "Y" => Play::Paper,
        "Z" => Play::Scissors,
        _ => {
            panic!("Unsupported Value");
        }
    };

    let their_play = match them {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        _ => {
            panic!("Unsupported Value");
        }
    };

    (my_play, their_play)
}

fn get_outcome_from_str(outcome: &str) -> RoundOutcome {
    match outcome {
        "X" => RoundOutcome::Lose,
        "Y" => RoundOutcome::Draw,
        "Z" => RoundOutcome::Win,
        _ => {
            panic!("Unsupported Value");
        }
    }
}

// Returns what I will play based on what they played, and the desired outcome
// for this round
fn get_plays_part_2(them: &str, desired_outcome: &RoundOutcome) -> Play {
    // let my_play = match me {
    //     "X" => Play::Rock,
    //     "Y" => Play::Paper,
    //     "Z" => Play::Scissors,
    //     _ => {
    //         panic!("Unsupported Value");
    //     }
    // };

    match them {
        "A" => match desired_outcome {
            RoundOutcome::Win => Play::Paper,
            RoundOutcome::Lose => Play::Scissors,
            RoundOutcome::Draw => Play::Rock,
        },
        "B" => match desired_outcome {
            RoundOutcome::Win => Play::Scissors,
            RoundOutcome::Lose => Play::Rock,
            RoundOutcome::Draw => Play::Paper,
        },
        "C" => match desired_outcome {
            RoundOutcome::Win => Play::Rock,
            RoundOutcome::Lose => Play::Paper,
            RoundOutcome::Draw => Play::Scissors,
        },
        _ => {
            panic!("Unsupported Value");
        }
    }
}

fn get_round_outcome(me: &Play, them: &Play) -> RoundOutcome {
    use crate::RoundOutcome::Draw;
    use crate::RoundOutcome::Lose;
    use crate::RoundOutcome::Win;

    match me {
        Play::Rock => match them {
            Play::Rock => Draw,
            Play::Paper => Lose,
            Play::Scissors => Win,
        },
        Play::Paper => match them {
            Play::Rock => Win,
            Play::Paper => Draw,
            Play::Scissors => Lose,
        },
        Play::Scissors => match them {
            Play::Rock => Lose,
            Play::Paper => Win,
            Play::Scissors => Draw,
        },
    }
}

fn get_me_value(me: &Play) -> usize {
    match me {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    }
}
