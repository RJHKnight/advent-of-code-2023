use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = Path::new("input.txt");
    let mut grid = parse_file(&file_path).unwrap();
    let mut prev_score = 0;

    for cycle in 0..1000 {
        
        for j in 0..grid[0].len() {
            tilt_col_north(&mut grid, j);
        }

        for i in 0..grid.len() {
            tilt_row_west(&mut grid, i);
        }

        for j in 0..grid[0].len() {
            tilt_col_south(&mut grid, j);
        }

        for i in 0..grid.len() {
            tilt_row_east(&mut grid, i);
        }

        let score = get_score(&grid);
        println!("Cycle: {}, Score: {}", cycle, score);
    }

    //print_grid(&grid);
    println!("Score: {}", get_score(&grid));
}

fn get_score(grid: &Vec<Vec<SquareType>>) -> i32 {
    let mut score = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == SquareType::Round {
                score += (grid.len() - i) as i32;
            }
        }
    }
    score
}

// Print the grid
fn print_grid(grid: &Vec<Vec<SquareType>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!("{}", match grid[i][j] {
                SquareType::Empty => '.',
                SquareType::Square => '#',
                SquareType::Round => 'O',
            });
        }
        println!();
    }
}

fn tilt_col_north(grid: &mut Vec<Vec<SquareType>>, col: usize) {

    for i in 1..grid.len() {
        if grid[i][col] == SquareType::Round {
            for j in (0..i).rev() {
                if grid[j][col] == SquareType::Empty {
                    grid[j][col] = SquareType::Round;
                    grid[j + 1][col] = SquareType::Empty;
                } else {
                    break;
                }
            }
        }
    }
}

fn tilt_col_south(grid: &mut Vec<Vec<SquareType>>, col: usize) {
    
    for i in (0..grid.len() - 1).rev() {
        if grid[i][col] == SquareType::Round {
            for j in i + 1..grid.len() {
                if grid[j][col] == SquareType::Empty {
                    grid[j][col] = SquareType::Round;
                    grid[j - 1][col] = SquareType::Empty;
                } else {
                    break;
                }
            }
        }
    }
}

fn tilt_row_east(grid: &mut Vec<Vec<SquareType>>, row: usize) {
    for j in (0..grid[row].len() - 1).rev() {
        if grid[row][j] == SquareType::Round {
            for k in j + 1..grid[row].len() {
                if grid[row][k] == SquareType::Empty {
                    grid[row][k] = SquareType::Round;
                    grid[row][k - 1] = SquareType::Empty;
                } else {
                    break;
                }
            }
        }
    }
}

fn tilt_row_west(grid: &mut Vec<Vec<SquareType>>, row: usize) {
    for j in 1..grid[row].len() {
        if grid[row][j] == SquareType::Round {
            for k in (0..j).rev() {
                if grid[row][k] == SquareType::Empty {
                    grid[row][k] = SquareType::Round;
                    grid[row][k + 1] = SquareType::Empty;
                } else {
                    break;
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum SquareType {
    Empty,
    Square,
    Round,
}

fn parse_file(file_path: &Path) -> io::Result<Vec<Vec<SquareType>>> {
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut row = Vec::new();

        for ch in line.chars() {
            let square = match ch {
                'O' => SquareType::Round,
                '#' => SquareType::Square,
                '.' => SquareType::Empty,
                _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid character")),
            };

            row.push(square);
        }

        grid.push(row);
    }

    Ok(grid)
}