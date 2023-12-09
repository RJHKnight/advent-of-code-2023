use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = parse_file_to_vec("input.txt").unwrap();

    let mut sum_next = 0;
    for line in lines {
        let next_val = get_next_number(&line);
        sum_next += next_val;
        //println!("Next value: {} for {:?}", next_val, line)

    }

    println!("Sum of next numbers: {}", sum_next);
}

fn get_next_number(line :&Vec<i32>) -> i32 {
    
    let mut lines = Vec::new();
    lines.push(line.clone());

    loop {
        // get last value in lines
        let last_line = lines.last().unwrap();
        let mut diff = Vec::new();
        // Calculate diff between neighbouring values in last line
        for i in 0..last_line.len()-1 {
            diff.push(last_line[i+1] - last_line[i]);
        }

        // Add diff to lines
        lines.push(diff.clone());

        // If all diff values are 0, break
        if diff.iter().all(|&x| x == 0) {
            break;
        }
    }

    let mut last_line_value = 0;

    // Reverse iterate through lines and push the sum of the last line to the next line
    for i in (0..lines.len()).rev() {
        let this_first_val = lines[i].first().unwrap().clone();

        let new_val = this_first_val - last_line_value;
        // Add new value to the start of the line
        lines[i].insert(0, new_val);

        last_line_value = new_val;
        // print line in red
        //println!("\x1b[31m{:?}\x1b[0m", lines[i]);
    }

    // Return the last value in the first line
    lines[0].first().unwrap().clone()

}

fn parse_file_to_vec(file_name: &str) -> io::Result<Vec<Vec<i32>>> {
    let path = Path::new(file_name);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line.split_whitespace()
                                    .map(|s| s.parse().unwrap())
                                    .collect();
        result.push(numbers);
    }

    Ok(result)
}