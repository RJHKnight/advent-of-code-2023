use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut points = parse_file(Path::new("input.txt")).unwrap();
    let jumps = expand_points(&mut points);

    let mut total_distances = HashMap::new();

    // For each point, add the distance to all other points
    for point in &points {
        for other_point in &points {
            if other_point.label == point.label {
                continue;
            }
            // Check if we have already calculated the distance
            if !total_distances.contains_key(&(other_point.label, point.label)) {
                let dist = point.distance(other_point, &jumps, 1_000_000);
                total_distances.insert((point.label, other_point.label), dist);
            }
        }
    }
    
    println!("Total distance: {}", total_distances.values().sum::<usize>());
}

fn expand_points(points: &Vec<Point>) -> (Vec<usize>, Vec<usize>){
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    let mut to_add_y = Vec::new();

    for y in 0..=max_y {
        // If no points have this value of y, insert a new row
        if !points.iter().any(|p| p.y == y) {
            to_add_y.push(y);
        }
    }

    let mut to_add_x = Vec::new();

    for x in 0..=max_x {
        // If no points have this value of x, insert a new column
        if !points.iter().any(|p| p.x == x) {
            to_add_x.push(x);
        }
    }

    (to_add_x, to_add_y)
}

fn insert_row(points: &mut Vec<Point>, at_y: usize) {
    for point in points.iter_mut() {
        if point.y > at_y {
            point.y += 1;
        }
    }
}

fn insert_column(points: &mut Vec<Point>, at_x: usize) {
    for point in points.iter_mut() {
        if point.x > at_x {
            point.x += 1;
        }
    }
}

fn print_grid(points: &Vec<Point>) {
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let mut found = false;
            for point in points {
                if point.x == x && point.y == y {
                    print!("{}", point.label);
                    found = true;
                    break;
                }
            }
            if !found {
                print!(".");
            }
        }
        println!();
    }
}

struct Point {
    label: usize,
    x: usize,
    y: usize,
}

impl Point {
    fn distance(&self, other: &Point, jumps: &(Vec<usize>, Vec<usize>), scale :usize) -> usize {

        let mut dx = (self.x as i64 - other.x as i64).abs() as usize;
        // Step from self.x to other.x, if if crosses a value in jumps.0, add 1,000,000
        let mut x_jumps = 0;
        for x in self.x.min(other.x)..=self.x.max(other.x) {
            if jumps.0.contains(&x) {
                x_jumps += 1;
            }
        }

        if x_jumps > 0 {
            dx += (x_jumps * scale) - x_jumps;
        }

        let mut dy = (self.y as i64 - other.y as i64).abs() as usize;
        // Step from self.y to other.y, if if crosses a value in jumps.1, add 1,000,000
        let mut y_jumps = 0;
        for y in self.y.min(other.y)..=self.y.max(other.y) {
            if jumps.1.contains(&y) {
                y_jumps += 1;
            }
        }

        if y_jumps > 0 {
            dy += (y_jumps * scale) - y_jumps;
        }

        dx + dy
    }
}

fn parse_file(path: &Path) -> io::Result<Vec<Point>> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut points = Vec::new();
    let mut label = 1;

    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        for (x, value) in line.chars().enumerate() {
            if value != '#' {
                continue;
            }

            points.push(Point { label, x, y });
            label += 1;
        }
    }

    Ok(points)
}