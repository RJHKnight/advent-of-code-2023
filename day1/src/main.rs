use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::slice::SplitInclusiveMut;

fn parse_file(filename: &str) -> io::Result<Vec<Vec<i8>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;

        // Replace "one" with "1", "two" with "2", etc.
        let line = line.replace("one", "on1e");
        let line = line.replace("two", "tw2o");
        let line = line.replace("three", "thr3ee");
        let line = line.replace("four", "fo4ur");
        let line = line.replace("five", "fi5ve");
        let line = line.replace("six", "s6ix");
        let line = line.replace("seven", "sev7en");
        let line = line.replace("eight", "eig8ht");
        let line = line.replace("nine", "ni9ne");

        let mut line_result = Vec::new();

        for ch in line.chars() {
            if let Ok(num) = ch.to_string().parse::<i8>() {
                line_result.push(num);
            }
        }

        result.push(line_result);
    }

    Ok(result)
}

fn main() {
    let digits =  parse_file("input.txt").unwrap();

    let mut sum = 0;

    for line in digits {
        //println!("{:?}", line);
        // Get first and last digits of line
        let first = line.first().unwrap();
        let last = line.last().unwrap();

        // Make a i32 by contatenating first and last
        let mut num = first.to_string();
        num.push_str(&last.to_string());
        let num = num.parse::<i32>().unwrap();
        sum += num;
    }

    println!("Sum: {}", sum);
}
