use std::collections::HashSet;

use image; // Add this import statement
// Define a line with a start point and an end point
#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_on_line(&self, point: &Point) -> bool {
        let x_min = self.start.x.min(self.end.x);
        let x_max = self.start.x.max(self.end.x);
        let y_min = self.start.y.min(self.end.y);
        let y_max = self.start.y.max(self.end.y);
        point.x >= x_min && point.x <= x_max && point.y >= y_min && point.y <= y_max
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: u64
}

impl Point {
    fn shift(& self, x: i64, y: i64) -> Point {
        Point { x: self.x + x, y: self.y + y }
    }
}

fn get_lines(instructions: &Vec<Instruction>) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut current_point = Point { x: 0, y: 0 };
    for instruction in instructions {
        let mut new_point = current_point;
        match instruction.direction {
            Direction::Right => new_point.x += instruction.steps as i64,
            Direction::Down => new_point.y -= instruction.steps as i64,
            Direction::Left => new_point.x -= instruction.steps as i64,
            Direction::Up => new_point.y += instruction.steps as i64,
        }
        lines.push(Line { start: current_point, end: new_point });
        current_point = new_point;
    }
    lines
}

fn get_area(lines: &Vec<Line>, start : Point) -> i64 {

    // Find lowest values of x and y across all lines
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    
    for line in lines {
        x_min = x_min.min(line.start.x.min(line.end.x));
        x_max = x_max.max(line.start.x.max(line.end.x));
        y_min = y_min.min(line.start.y.min(line.end.y));
        y_max = y_max.max(line.start.y.max(line.end.y));
    }

    let mut stack = Vec::new();
    let mut visited = HashSet::new();

    let mut count: i64 = 1;
    visited.insert(start);

    // Starting at the start, add neighbouring points to the stack
    stack.push(start.shift(1, 0));
    stack.push(start.shift(-1, 0));
    stack.push(start.shift(0, 1));
    stack.push(start.shift(0, -1));

    while !stack.is_empty() {
        let point = stack.pop().unwrap();

        // If point is outside the bounds, panic
        if point.x < x_min || point.x > x_max || point.y < y_min || point.y > y_max {
            panic!("Point outside bounds");
        }

        if visited.contains(&point) {
            continue;
        }
        
        if lines.iter().any(|line| line.is_on_line(&point)) {
            continue;
        }

        visited.insert(point);
        count += 1;
        stack.push(point.shift(1, 0));
        stack.push(point.shift(-1, 0));
        stack.push(point.shift(0, 1));
        stack.push(point.shift(0, -1));
    }

    // Add on the values of the lines
    for line in lines {
        let x_min = line.start.x.min(line.end.x);
        let x_max = line.start.x.max(line.end.x);
        let y_min = line.start.y.min(line.end.y);
        let y_max = line.start.y.max(line.end.y);

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let point = Point { x, y };
                if visited.contains(&point) {
                    continue;
                }
                count += 1;
                visited.insert(point);
            }
        }
    }
    count

}



fn parse_instructions_step2(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let hex = parts[2];
        
        let hex = hex.trim_start_matches('(').trim_end_matches(')');
        let steps = u64::from_str_radix(&hex[1..6], 16).unwrap();
        let direction_int = u8::from_str_radix(&hex[6..7], 16).unwrap();
        let direction = match direction_int {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Invalid direction"),
        };
        Instruction { direction, steps}
    }).collect()
}


fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let direction = match parts[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let steps = parts[1].parse().unwrap();

        Instruction { direction, steps }
    }).collect()
}

// Create a JPEG from Vec<Line>
fn create_jpeg(lines: &Vec<Line>) {
    
    // Find lowest values of x and y across all lines
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    
    for line in lines {
        x_min = x_min.min(line.start.x.min(line.end.x));
        x_max = x_max.max(line.start.x.max(line.end.x));
        y_min = y_min.min(line.start.y.min(line.end.y));
        y_max = y_max.max(line.start.y.max(line.end.y));
    }
    
    let mut imgbuf = image::ImageBuffer::<image::Rgb<u8>, Vec<u8>>::new(1000,1000);

    let mut counter = 0;

    for line in lines {
        let this_x_min = line.start.x.min(line.end.x);
        let this_x_max = line.start.x.max(line.end.x);
        let this_y_min = line.start.y.min(line.end.y);
        let this_y_max = line.start.y.max(line.end.y);

        for x in this_x_min..=this_x_max {
            for y in this_y_min..=this_y_max {
                let colour = if counter < 10 { image::Rgb([0, 255, 0]) } else { image::Rgb([0, 0, 255]) };

                imgbuf.put_pixel((x - x_min) as u32, (y - y_min) as u32, colour);
                counter += 1;
            }
        }
    }
    imgbuf.save("test.jpeg").unwrap();
}

fn main() {
    let input = include_str!("../input_test.txt");
    let instructions = parse_instructions_step2(input);
    let lines = get_lines(&instructions);
    //create_jpeg(&lines);
    
    // let area = get_area(&lines, Point { x: 0, y: -1 });
    let area = get_area(&lines, Point { x: 1, y: -1 });
    // let area = get_area(&lines, Point { x: 1, y: 0 });
    // let area = get_area(&lines, Point { x: -1, y: 0 });
    println!("Area: {}", area);
}
