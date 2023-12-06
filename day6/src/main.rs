use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_name = "input.txt";
    let race = parse_file(file_name);

    println!("Races {:?}", race);

    let score = get_winning_strat_count(race);

    println!("Score: {}", score);
}

fn get_winning_strat_count(details :(i64, i64)) -> i64 {
    let mut winning_count = 0;

    let time = details.0;
    let distance = details.1;

    for hold_time in 1..time-1 {
        let this_distance = hold_time * (time - hold_time);
        if this_distance > distance {
            winning_count += 1;
        }
    }

    winning_count
}


fn parse_file(file_name: &str) -> (i64, i64) {
    let path = Path::new(file_name);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();

    // Skip the first entry, then concatenate the strings, then parse to i32
    let time :i64 = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |mut acc, s| {
            acc.push_str(s.trim());
            acc
        })
        .parse()
        .unwrap();

    let distance :i64 = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |mut acc, s| {
            acc.push_str(s.trim());
            acc
        })
        .parse()
        .unwrap();

    (time, distance)
}