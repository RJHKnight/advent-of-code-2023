use std::collections::HashMap;

fn main() {
    // Read input to string
    let input = include_str!("../input.txt");
    let puzzle = parse_input(input);

    let (directions, maze) = puzzle;

    let mut steps = 1;

    // Current is a vec of all values in maze.keys that start with "A"
    let mut current: Vec<&str> = maze.keys().filter(|k| k.ends_with("A")).map(|k| *k).collect();

    println!("Starting values: {:?}", current);
    let mut instruction_pointer = 0;

    let mut steps_per_value = HashMap::new();

    // Loop until keys of steps-per-value contains all values in current
    //while current.len() > 0 {
    for _ in 1..100000 {
        
        let direction = &directions[instruction_pointer];

        let previous = current.clone();
        current.clear();

        for key in previous.iter() {
            let (next_left, next_right) = maze.get(key).unwrap();
            let next = match direction {
                Direction::Right => next_right,
                Direction::Left => next_left,
            };

            if next.ends_with("Z") {
                println!("Found Z for {}!", key);
                let steps_vec = steps_per_value.entry(key.to_owned()).or_insert(Vec::new());
                steps_vec.push(steps);
            }
            //else {
                current.push(next);
            //}
        }

        steps += 1;

        if instruction_pointer == directions.len() - 1 {
            instruction_pointer = 0;
        } else {
            instruction_pointer += 1;
        }   
    }

    println!("Steps per value: {:?}", steps_per_value);

    // Collect first element of each value in steps_per_value
    let mut steps_vec: Vec<u128> = steps_per_value.values().map(|v| v[0]).collect();

    // println!("Steps per value: {:?}", steps_per_value);
    // Find the lowest value that is a common multiple of all values in steps_per_value
    let mut lowest_common_multiple = 1;
    for value in steps_vec.iter() {
        lowest_common_multiple = lcm(lowest_common_multiple, *value);
        println!("Lowest common multiple: {}", lowest_common_multiple)
    }

    println!("Lowest common multiple: {}", lowest_common_multiple);
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, (&str, &str)>) {
    let lines: Vec<&str> = input.lines().collect();
    let directions: Vec<Direction> = lines[0].chars().map(|c| {
        match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Invalid direction"),
        }
    }).collect();

    let mut maze: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines[2..].iter() {
        let parts: Vec<&str> = line.split(" = ").collect();
        let values: Vec<&str> = parts[1][1..parts[1].len()-1].split(", ").collect();
        maze.insert(parts[0], (values[0], values[1]));
    }

    (directions, maze)
}