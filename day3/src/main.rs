use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main()  {
    let grid = parse("input.txt");

    let mut gear_ratios = Vec::new();
    let mut gear_index: Vec<(usize, usize)> = Vec::new();
    
    for y in 0..grid.len() {
        let line = &grid[y];
        let mut digits = Vec::new();
        let mut handling_number = false;
        let mut next_digit = None;
        
        for x in 0..line.len() {

            if y == 10 && x == line.len() - 10 {
                print!("");
            }
            let point = line[x];
            let is_digit = point.is_digit(10);

            if is_digit {
                handling_number = true;
                digits.push(point);
                if next_digit.is_none() {
                    next_digit = check_surrounding(&grid, x, y);
                }
            }
            if (!is_digit || (x == line.len() - 1)) & handling_number {
                handling_number = false;
                
                if next_digit.is_some() {
                    // Concat digits and convert to u32
                    let mut number = String::new();
                    for digit in &digits {
                        number.push(digit.clone());
                    }
                    let number = number.parse::<u64>().unwrap();

                    let (next_x, next_y) = next_digit.unwrap();

                    let other_number = get_number_around(&grid, next_x, next_y, x-1, y, &mut gear_index);

                    if other_number.is_some() {
                        gear_ratios.push((number,other_number.unwrap()));
                    }
                    next_digit = None;
                } 

                digits.clear();
            }
        }
    }

    print_grid(&grid, &gear_index);

    // sum valid ids    
   let sum = gear_ratios.iter().map(|x| x.0 * x.1).sum::<u64>();
   println!("Valid ids: {:?}", gear_ratios);
   println!("Sum of gear ratios: {}", sum);

}

// Print out the grid, highlighting in red the points in gear_index
fn print_grid(grid: &Vec<Vec<char>>, gear_index: &Vec<(usize, usize)>) {
    for y in 0..grid.len() {
        let line = &grid[y];
        for x in 0..line.len() {
            let point = line[x];
            let mut is_gear = false;
            for (gear_x, gear_y) in gear_index {
                if *gear_x == x && *gear_y == y {
                    is_gear = true;
                    break;
                }
            }
            if is_gear {
                print!("\x1b[31m{}\x1b[0m", point);
            }
            else {
                print!("{}", point);
            }
        }
        println!();
    }
}

// Directions, including diagonals
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), // top left
    (-1, 0),  // top
    (-1, 1),  // top right
    (0, -1),  // left
    (0, 1),   // right
    (1, -1),  // bottom left
    (1, 0),   // bottom
    (1, 1),   // bottom right
];

// Check if the point is surrounded by a * that is not a digit
fn check_surrounding(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Option<(usize, usize)> {
    
    for (dx, dy) in DIRECTIONS.iter() {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        if new_x < 0 || new_y < 0 {
            continue;
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        if new_y >= grid.len() || new_x >= grid[y].len() {
            continue;
        }

        let this_check = check_point(new_x, new_y, grid);

        if this_check.is_some() {
            return this_check;
        }
    }

    None
}

// function to check if the point is not a digit or a dot
fn check_point(x: usize, y: usize, grid: &Vec<Vec<char>>,) -> Option<(usize, usize)> {
    
    let point = grid[y][x];
    if point != '*' {
        return None;
    }

    for (dx, dy) in DIRECTIONS.iter() {
        let x = x as i32 + dx;
        let y = y as i32 + dy;

        if x < 0 || y < 0  {
            continue;
        }

        let x = x as usize;
        let y = y as usize;

        if y >= grid.len() || x >= grid[y].len() {
            continue;
        }

        let this_check = (grid[y][x]).is_digit(10);

        if this_check{
            return Some((x, y));
        }
    }

    None
    
}

fn get_number_around(grid: &Vec<Vec<char>>, x: usize, y: usize, 
    this_x: usize, this_y: usize,
    gear_index: &mut Vec<(usize, usize)>) -> Option<u64> {
    
    // Starting from point x,y in the grid, find the first value to the left that isnt a digit
    // Then find the first value to the right that isnt a digit
    // Then return all the digits in between as a u64
    let mut left_x = x;
    while (left_x > 0) && (grid[y][left_x-1].is_digit(10)) {
        left_x -= 1;
    }

    let mut right_x = x;
    while (right_x < grid[y].len()-1) && (grid[y][right_x+1].is_digit(10)) {
        right_x += 1;
    }
    
    let mut to_add = Vec::new();

    let mut number = String::new();
    for x in left_x..right_x+1 {

        if (x == this_x) && (y == this_y) {
            return None;
        }

        to_add.push((x, y));
        number.push(grid[y][x]);
    }

    gear_index.append(&mut to_add);

    Some(number.parse::<u64>().unwrap())
}


fn parse(file_path: &str) -> Vec<Vec<char>> {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    grid
}
