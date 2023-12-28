use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input = parse_input("input_test.txt").unwrap();

    let input_expanded = unfold_input(&input);

    let mut valid_count = 0;

    for (springs, numbers) in input_expanded {

        let spring_perms = get_spring_perms(&springs);
        let num_spring_perms = spring_perms.len();
        let mut this_spring_count = 0;

        //print_springs(&springs);
        //println!("Numbers: {:?}", numbers);
        println!("Number of spring permutations: {}.", num_spring_perms);
        let sum_numbers = numbers.iter().sum::<usize>();

        let mut count = 0;

        for springs in spring_perms {

            println!("Trying permutation: {} ", count);

            if count == 524287 {
                println!("Springs: {:?}", springs);
                println!("Numbers: {:?}", numbers);
            }
            count += 1;

            // Check sum permutation = sum numbers
            let sum_springs = springs.iter().map(|s| if *s { 1 } else { 0 }).sum::<usize>();

            if sum_springs != sum_numbers {
                continue;
            }
            
            let is_valid = validate_springs(&springs, &numbers);
            
            if is_valid {
                this_spring_count += 1;
            }
            // If is valid print the springs in green, else in red
            // print!("\x1B[{}m", if is_valid { 32 } else { 31 });
            // println!("Testing permutation: {:?}", springs);
            // print!("\x1B[0m");
        }
        //println!("Valid count: {}.", this_spring_count);
        //println!();

        valid_count += this_spring_count;
    }

    println!("*** Total Valid count: {}", valid_count);
}

fn unfold_input(input: &Vec<(Vec<Option<bool>>, Vec<usize>)>) -> Vec<(Vec<Option<bool>>, Vec<usize>)> {
   
   let mut res = Vec::new();

    // Iterate over input
    for (springs, numbers) in input {
        
        // Make 5 copies of the springs, adding a None to the end of each
        let mut new_springs = Vec::new();

        for i in 0..5 {
            let mut copy = springs.clone();
            if i < 4  { 
                copy.push(None);
            }

            new_springs.append(&mut copy);
        }

        let mut new_numbers = Vec::new();

        for i in 0..5 {
            let mut copy = numbers.clone();
            new_numbers.append(&mut copy);
        }

        res.push((new_springs, new_numbers));
    }

   res
   
}

fn print_springs(springs: &Vec<Option<bool>>) {
    for spring in springs {
        if spring.is_none() {
            print!("?");
        } else
        if spring.unwrap() {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!("");
}


fn get_spring_perms(springs: &Vec<Option<bool>>) -> Vec<Vec<bool>> {
    
    let mut result = Vec::new();

    // If there are no more None values, return the current springs
    if !springs.contains(&None) {
        result.push(springs.iter().map(|s| s.unwrap()).collect());
        return result;
    }

    // Calculate each permutation where Option::None is replaced with each of true and false
    for i in 0..springs.len() {
        if let Some(_) = springs[i] {
            continue;
        }

        let mut new_springs = springs.clone();
        new_springs[i] = Some(true);
        result.append(&mut get_spring_perms(&new_springs));

        let mut new_springs = springs.clone();
        new_springs[i] = Some(false);
        result.append(&mut get_spring_perms(&new_springs));

        break;
    }

    result
}

fn validate_springs(springs: &Vec<bool>, numbers: &[usize]) -> bool {
    
    let mut running_count = 0;
    let mut numbers_index = 0;

    for spring in springs {
        if *spring {
            running_count += 1;
        } else {

            if running_count == 0 {
                continue;
            }

            if numbers_index < numbers.len() && running_count == numbers[numbers_index] {
                numbers_index += 1;
                running_count = 0;
            } 
            else if numbers_index < numbers.len() && running_count > numbers[numbers_index] {
                return false;
            }else {
                return false;
            }
        }
    }

    if numbers_index != numbers.len() {
        if numbers_index != numbers.len()-1 || running_count != numbers[numbers_index] {
            return false;
        }
        return true;
    }

    if running_count > 0 {
        if numbers_index > numbers.len()-1 || running_count != numbers[numbers_index] {
            return false;
        }
    }

    true
}

fn parse_input(file_path: &str) -> io::Result<Vec<(Vec<Option<bool>>, Vec<usize>)>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        let springs: Vec<Option<bool>> = parts[0].chars().map(|c| match c {
            '#' => Some(true),
            '.' => Some(false),
            _ => None,
        }).collect();

        let numbers: Vec<usize> = parts[1].split(',').filter_map(|s| s.parse().ok()).collect();

        result.push((springs, numbers));
    }

    Ok(result)
}

// test case for get_spring_perms with None,None,None,F,T,T,T
#[test]
fn test_get_spring_perms() {
    let springs = vec![None, None, None, Some(false), Some(true), Some(true), Some(true)];
    let mut perms = get_spring_perms(&springs);
    perms.sort();
    perms.reverse();

    assert_eq!(perms.len(), 8);
    assert_eq!(perms[0], vec![true, true, true, false, true, true, true]);
    assert_eq!(perms[1], vec![true, true, false, false, true, true, true]);
    assert_eq!(perms[2], vec![true, false, true, false, true, true, true]);
    assert_eq!(perms[3], vec![true, false, false, false, true, true, true]);
    assert_eq!(perms[4], vec![false, true, true, false, true, true, true]);
    assert_eq!(perms[5], vec![false, true, false, false, true, true, true]);
    assert_eq!(perms[6], vec![false, false, true, false, true, true, true]);
    assert_eq!(perms[7], vec![false, false, false, false, true, true, true]);
}

// Test case for is_valid with [false, false, true, false, false, true, false, false, false, false, true, true, true, false]
#[test]
fn test_is_valid() {
    let springs = vec![false, false, true, false, false, true, false, false, false, false, true, true, true, false];
    let numbers = vec![1, 1, 3];

    assert_eq!(validate_springs(&springs, &numbers), true);
}

// test is valid with [false, true, true, true, false, false, true, true, false, false, false, true]
#[test]
fn test_is_valid_2() {
    let springs = vec![false, true, true, true, false, false, true, true, false, false, false, true];
    let numbers = vec![3, 2, 1];

    assert_eq!(validate_springs(&springs, &numbers), true);
}

// test is valid with [false, false, false, false, true, true, true, false, false, false, false, false, true, true, true, false, false, false, false, false, true, true, true, false, false, false, false, false, true, true, true, false, false, false, false, false, true, true, true]
#[test]
fn test_is_valid_3() {
    let springs =
        vec![false, false, false, false, true, true, true, false, false, false, false, false, true, true, true, false, false,
        false, false, false, true, true, true, false, false, false, false, false, true, true, true, false, false, false,
        false, false, true, true, true];

    let numbers = vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3];
    assert_eq!(validate_springs(&springs, &numbers), true);
}

