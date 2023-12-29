use std::collections::VecDeque;
use rayon::prelude::*;

// Define the enum
#[derive(Debug)]
enum SquareType {
    None,
    SplitterVert,
    SplitterHoriz,
    MirrorForward,
    MirrorBack,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Function to convert a character to SquareType
fn char_to_square_type(c: char) -> SquareType {
    match c {
        '|' => SquareType::SplitterVert,
        '-' => SquareType::SplitterHoriz,
        '/' => SquareType::MirrorForward,
        '\\' => SquareType::MirrorBack,
        _ => SquareType::None,
    }
}

// Function to parse the input file
fn parse_input(input: &str) -> Vec<Vec<SquareType>> {
    input
        .lines()
        .map(|line| line.chars().map(char_to_square_type).collect())
        .collect()
}

// safe setter for grid, handles out of bounds
fn increase_val(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    if i < grid.len() && j < grid[0].len() {
        grid[i][j] = grid[i][j] + 1;
    }
}

fn follow_path(grid: &Vec<Vec<SquareType>>, entry_point: (usize, usize, Direction)) -> Vec<Vec<i32>> {
    
    let mut res = vec![vec![0; grid[0].len()]; grid.len()];
    // To visit should be a fifo queue


    let mut to_visit = VecDeque::new();
    to_visit.push_back(entry_point.clone());

    let mut first = true;
    let mut counter = 0;

    while !to_visit.is_empty() && counter < 50000000 {
        
        counter += 1;
        let (i, j, dir) = to_visit.pop_front().unwrap();
        
        //println!("x={} y={} dir={:?}", j, i, dir);

        increase_val(&mut res, i, j);

        if i==0 && j ==0 && !first {
            first = false;
            continue;
        }

        match grid[i][j] {

            SquareType::SplitterHoriz => {
                match dir {
                    Direction::Up | Direction::Down => {
                        if j > 0 {
                            to_visit.push_back((i, j-1, Direction::Left));
                        }
                        if j < res[0].len()-1 {
                            to_visit.push_back((i, j+1, Direction::Right));
                        }
                    },
                    Direction::Left => if j > 0 {to_visit.push_back((i, j-1, Direction::Left)) },
                    Direction::Right => if j < res[0].len()-1  {to_visit.push_back((i, j+1, Direction::Right))},
                }
            },

            SquareType::SplitterVert => {
                match dir {
                    Direction::Left | Direction::Right => {
                        if i > 0 {
                            to_visit.push_back((i-1, j, Direction::Up));
                        }
                        if i < res.len()-1 {
                            to_visit.push_back((i+1, j, Direction::Down));
                        }
                    },
                    Direction::Up => if i > 0 {to_visit.push_back((i-1, j, Direction::Up)) },
                    Direction::Down => if i < res.len()-1  {to_visit.push_back((i+1, j, Direction::Down))},
                }
            },
            // / 
            SquareType::MirrorForward => {
                match dir {
                    Direction::Down => {
                        // Left
                        if j > 0 {
                            to_visit.push_back((i, j-1, Direction::Left));
                        }
                    },
                    Direction::Up => {
                        // Right
                        if j < res[0].len()-1 {
                            to_visit.push_back((i, j+1, Direction::Right));
                        }
                    },
                    Direction::Left => {
                        // Down
                        if i < res.len()-1 {
                            to_visit.push_back((i+1, j, Direction::Down));
                        }
                    },
                    Direction::Right => {
                        // Up
                        if i > 0 {
                            to_visit.push_back((i-1, j, Direction::Up));
                        }
                    },
                }
            },
            // \
            SquareType::MirrorBack => {
                match dir {
                    Direction::Down => {
                        // Right
                        if j < res[0].len()-1 {
                            to_visit.push_back((i, j+1, Direction::Right));
                        }
                    },
                    Direction::Up => {
                        // Left
                        if j > 0 {
                            to_visit.push_back((i, j-1, Direction::Left));
                        }
                    },
                    Direction::Left => {
                        // Up
                        if i > 0 {
                            to_visit.push_back((i-1, j, Direction::Up));
                        }
                    },
                    Direction::Right => {
                        // Down
                        if i < res.len()-1 {
                            to_visit.push_back((i+1, j, Direction::Down));
                        }
                    },
                }
            },
            SquareType::None => {
                match dir {
                    Direction::Up => if i > 0 {to_visit.push_back((i-1, j, Direction::Up)) },
                    Direction::Down => if i < res.len()-1  {to_visit.push_back((i+1, j, Direction::Down))},
                    Direction::Left => if j > 0 {to_visit.push_back((i, j-1, Direction::Left)) },
                    Direction::Right => if j < res[0].len()-1  {to_visit.push_back((i, j+1, Direction::Right))},
                }
            },
        }

    }

    res
}

fn main () {
    let input = include_str!("../input.txt");
    let grid = parse_input(input);

    // Try all of the edge points
    let mut entry_points = Vec::new();

    // Corners

    // Top left
    entry_points.push((0, 0, Direction::Right));
    entry_points.push((0, 0, Direction::Down));

    // Bottom left
    entry_points.push((grid.len()-1, 0, Direction::Right));
    entry_points.push((grid.len()-1, 0, Direction::Up));

    // Top right
    entry_points.push((0, grid[0].len()-1, Direction::Left));
    entry_points.push((0, grid[0].len()-1, Direction::Down));

    // Bottom right
    entry_points.push((grid.len()-1, grid[0].len()-1, Direction::Left));
    entry_points.push((grid.len()-1, grid[0].len()-1, Direction::Up));

    // Edges
    for i in 1..grid[0].len()-1 {
        // Top
        entry_points.push((0, i, Direction::Down));
        // Bottom
        entry_points.push((grid.len()-1, i, Direction::Up));
    }

    for i in 1..grid.len()-1 {
        // Left
        entry_points.push((i, 0, Direction::Right));
        // Right
        entry_points.push((i, grid[0].len()-1, Direction::Left));
    }

    let max_count = entry_points.par_iter()
        .map(|entry_point| {
            println!("Trying entry Point: {:?}", entry_point);
            let res = follow_path(&grid, *entry_point);

            // Count number of vals in res > 0
            let mut count = 0;
            for i in 0..res.len() {
                for j in 0..res[0].len() {
                    if res[i][j] > 0 {
                        count += 1;
                    }
                }
            }

            count
        })
        .max()
        .unwrap_or(0);

    println!("Max Count: {}", max_count);
}