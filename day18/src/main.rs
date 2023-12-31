#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn is_black(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }

    // is white
    fn is_white(&self) -> bool {
        self.r == 255 && self.g == 255 && self.b == 255
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: u64,
    color: Color,
}

struct Grid {
    grid: Vec<Vec<Color>>,
    position: (usize, usize),
}

// Ctor for Grid
impl Grid {
    fn new() -> Grid {
        Grid {
            // Empty grid
            grid: {
                // Create a 100x100 grid
                let mut grid = Vec::with_capacity(1000);
                for _ in 0..1000 {
                    let mut row = Vec::with_capacity(1000);
                    for _ in 0..1000 {
                        row.push(Color { r: 0, g: 0, b: 0 });
                    }
                    grid.push(row);
                }
                grid
            },
            position: (500, 500),
        }
    }

    // fn to set a colour at a position, adding an empty row or column if needed
    fn set_color(&mut self, color: &Color, position: (usize, usize)) {
        let (y, x) = position;
        if y >= self.grid.len() {
            //println!("Adding row");
            // Add a new row, initialized to 0,0,0
            let row_size = self.grid[0].len();
            let new_row = vec![Color { r: 0, g: 0, b: 0 }; row_size];
            self.grid.push(new_row);
        }
        if x >= self.grid[0].len() {
            //println!("Adding column");
            for i in 0..self.grid.len() {
                self.grid[i].push(Color { r: 0, g: 0, b: 0 });
            } 
        }
        self.grid[y][x] = *color;
    }

    fn add_instruction(&mut self, instruction: &Instruction) {
        
        println!("Adding instruction: {:?}", instruction);
        let (dy, dx) = match instruction.direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        for _ in 0..instruction.steps {

            let (y, x) = self.position;
            let (y, x) = (y as i32 + dy, x as i32 + dx);
            let (y, x) = (y as usize, x as usize);
            self.position = (y, x);
            self.set_color(&instruction.color, self.position);
        }
    }

    fn follow_instructions(&mut self, instructions: &Vec<Instruction>) {
    
        for instruction in instructions {
            self.add_instruction(instruction);
        }
    
    }

    // Print the grid, using the colour
    fn print(&self) {
        for row in &self.grid {
            for color in row {
                print!("\x1b[48;2;{};{};{}m  ", color.r, color.g, color.b);
            }
            println!("\x1b[0m");
        }
    }

    // Create a jpeg of the grid
    fn jpeg(&self) {
        let mut imgbuf = image::ImageBuffer::new(self.grid[0].len() as u32, self.grid.len() as u32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let color = &self.grid[y as usize][x as usize];
            *pixel = image::Rgb([color.r, color.g, color.b]);
        }
        imgbuf.save("image.jpg").unwrap();
    }

    // Flood fill any black grid cells enclosed by any colour with white
    fn flood_fill(&mut self) {
        let mut visited = vec![vec![false; self.grid[0].len()]; self.grid.len()];
        let mut queue = Vec::new();
        queue.push((0, 0));
        while !queue.is_empty() {
            let (y, x) = queue.pop().unwrap();
            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;
            if !self.grid[y][x].is_black() {
                continue;
            }
            self.grid[y][x] = Color { r: 255, g: 255, b: 255 };
            if y > 0 {
                queue.push((y - 1, x));
            }
            if y < self.grid.len() - 1 {
                queue.push((y + 1, x));
            }
            if x > 0 {
                queue.push((y, x - 1));
            }
            if x < self.grid[0].len() - 1 {
                queue.push((y, x + 1));
            }
        }
    }
}


fn parse_color(color: &str) -> Color {
    let color = color.trim_start_matches('(').trim_end_matches(')');
    let r = u8::from_str_radix(&color[1..3], 16).unwrap();
    let g = u8::from_str_radix(&color[3..5], 16).unwrap();
    let b = u8::from_str_radix(&color[5..7], 16).unwrap();

    Color { r, g, b }
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
        let color = parse_color(parts[2]);

        Instruction { direction, steps, color }
    }).collect()
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
        let white = Color { r: 255, g: 255, b: 255 };
        Instruction { direction, steps,  color: white}
    }).collect()
}

fn main() {
    let input = include_str!("../input_test.txt");
    let instructions = parse_instructions_step2(input);
    let mut grid = Grid::new();
    grid.follow_instructions(&instructions);
    grid.flood_fill();
    //grid.print();    
    // Count the number of cells that are not white
    let count = grid.grid.iter().map(|row| {
        row.iter().filter(|color| !color.is_white()).count()
    }).sum::<usize>();

    //grid.jpeg();

    println!("Count: {}", count);
}
