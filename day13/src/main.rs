use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let grids = parse_file("input.txt");

    let mut total = 0;
    for i in 0..grids.len() {
        
        let grid = &grids[i];
        total += test_grid_with_mutations(grid);
    }

    println!("Total: {}", total);
}

fn test_grid_with_mutations(grid : &Vec<Vec<bool>>) -> i32 {

    // Prev score
    let prev_score = test_grid(&grid, true, usize::MAX).unwrap();

    //println!("Prev score: {}", prev_score.2);

    for y in 0..grid.len() {
        for x in 0..grid[0].len() { 
            let mut new_grid = grid.clone();
            // flip the value at x,y
            new_grid[y][x] = !new_grid[y][x];

            let score = test_grid(&new_grid, prev_score.0, prev_score.1);

            if score.is_some() {

                return score.unwrap().2;
            }
            
        }
    }

    0
}
 
fn test_grid(grid : &Vec<Vec<bool>>, is_horiz: bool, skip: usize)  -> Option<(bool, usize, i32)> {

    // First test horizontal
    for i in 1..grid.len() {

        if is_horiz && i == skip {
            continue;
        }
        let (left, right) = split_vec_bool(&grid, i, true);

        if compare_pair_horizontal(&left, &right) {
            return Some((true, i, 100 * i as i32));
        }
    }

    // Then test vertical
    for i in 1..grid[0].len() {

        if !is_horiz && i == skip {
            continue;
        }

        let (top, bottom) = split_vec_bool(&grid, i, false);

        if compare_pair_vertical(&top, &bottom) {
            return Some((false, i, i as i32));
        }
    }

    None
}
    

// Function to split Vec<Vec<bool>> about an x or y point, returning (top, bottom) or (left, right)
fn split_vec_bool(grid: &Vec<Vec<bool>>, point: usize, horizontal: bool) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    
    let mut top = Vec::new();
    let mut bottom = Vec::new();

    let end;
    let start;

    if horizontal {

        if (point as f32) > grid.len() as f32 /2f32 {
            // Reduce start
            end = grid.len();
            start = point - (grid.len()-point);
        } else if (point as f32) < grid.len() as f32/2f32 {
            // Reduce end
            end = 2*point;
            start = 0;
        }
        else {
            // Exactly in the middle
            end = grid.len();
            start = 0;
        }
    } else {
        let num_col = grid[0].len();
        if (point as f32) > num_col as f32 / 2f32 {
            // Reduce start
            end = num_col;
            start = point - (num_col-point);
        } else if (point as f32) < num_col as f32 / 2f32 {
            // Reduce end
            end = 2*point;
            start = 0;
        }
        else {
            // Exactly in the middle
            end = num_col;
            start = 0;
        }
    }

    if horizontal {
        for i in start..point {
            top.push(grid[i].clone());
        }
        for i in point..end {
            bottom.push(grid[i].clone());
        }
    } else {
        for i in 0..grid.len() {
            let mut row = Vec::new();
            for j in start..point {
                row.push(grid[i][j]);
            }
            top.push(row.clone());

            row = Vec::new();
            for j in point..end {
                row.push(grid[i][j]);
            }
            bottom.push(row);
        }
    }

    (top, bottom)
}

fn compare_pair_horizontal(a: &Vec<Vec<bool>>, b: &Vec<Vec<bool>>) -> bool {
    // Use zip to interate over a,b... calling compare_vec_bool on each pair
    for (a_row, b_row) in a.iter().zip(b.iter().rev()) {
        if !compare_vec_bool(a_row, b_row, false) {
            return false;
        }
    }

    true
}


fn compare_pair_vertical(a: &Vec<Vec<bool>>, b: &Vec<Vec<bool>>) -> bool {
    // Use zip to interate over a,b... calling compare_vec_bool on each pair
    for (a_row, b_row) in a.iter().zip(b.iter()) {
        if !compare_vec_bool(a_row, b_row, true) {
            return false;
        }
    }

    true
}

// function to compare two Vec<bool> and return true if they are equal
fn compare_vec_bool(a: &Vec<bool>, b: &Vec<bool>, reverse_b: bool) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for i in 0..a.len() {
        let b_index = if reverse_b {a.len()-i-1} else {i};
        if a[i] != b[b_index] {
            return false;
        }
    }

    true
}

fn parse_file(file_path: &str) -> Vec<Vec<Vec<bool>>> {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let mut result = Vec::new();

    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            result.push(grid.clone());
            grid = Vec::new();
            continue;
        }

        let mut row = Vec::new();
        for ch in line.chars() {
            match ch {
                '#' => row.push(true),
                '.' => row.push(false),
                _ => (),
            }
        }
        grid.push(row);
    }

    result.push(grid.clone());

    result
}
