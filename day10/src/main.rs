use std::collections::HashMap;

fn main() {
    let grid = parse_input(include_str!("../input.txt"));

    // // PRINT THE GRID
    // for row in &grid {
    //     for tile in row {
    //         print!("{:?} ", tile);
    //     }
    //     println!();
    // }
    
    // Find the start position
    let start_position = find_start_position(&grid);

    // Use a DFS to follow all the connected tiles until we have visted them all, keeping track of the number
    // of steps taken to reach each tile
    let mut visited = HashMap::new();
    visited.insert(start_position, 0);

    let mut longest_path: Option<Vec<Position>> = None;

    // For each direction, move to the next tile and add it to the stack if it not none
    for direction in &[Direction::S, Direction::E, Direction::N, Direction::W] {
        let path = follow_path(start_position, *direction, &grid);
        
        if let Some(path) = path {
                longest_path = Some(path);
        }
    }

    println!("Longest path: {:?}", longest_path.unwrap().len()/2);
    
}


fn follow_path(start_position: Position, initial_direction: Direction, grid: &Vec<Vec<TileType>>) -> Option<Vec<Position>> {

    println!("Starting path from {:?} in direction {:?}", start_position, initial_direction);
    let mut path = Vec::new();
    path.push(start_position);
    let mut position = start_position.move_position(initial_direction, grid);
    
    if position.is_some() {
        path.push(position.unwrap());
        let mut next_tile = get_tile_type(grid, position.unwrap());
        if next_tile.is_none() {
            return None;
        }
        
        loop {

            println!("Trying position {:?}", position.unwrap());

            let (p1, p2) = next_tile.unwrap().connect(position.unwrap(), grid);
            
            // Check if we are back at the start
            if (p1.is_some() && p1.unwrap() == start_position) || (p2.is_some() && p2.unwrap() == start_position) {
                if path.len() > 2 {
                    return Some(path);
                }
            }
            
            if p1.is_some() && path.contains(&p1.unwrap()) {
                if p2.is_none() || path.contains(&p2.unwrap()){
                    return None;
                }
                position = p2;
            } else if p2.is_some() && path.contains(&p2.unwrap()) {
                
                if p1.is_none() || path.contains(&p1.unwrap()){
                    return None;
                }
                position = p1;
            }
            else {
                return None;
            }

            next_tile = get_tile_type(grid, position.unwrap());

            if next_tile.is_none() {
                return None;
            }

            path.push(position.unwrap());
        }
    } 

    None
}

fn find_start_position(grid: &Vec<Vec<TileType>>) -> Position {
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == TileType::START {
                return Position { x, y };
            }
        }
    }
    panic!("No start position found");
}

fn get_tile_type(grid: &Vec<Vec<TileType>>, position: Position) -> Option<TileType> {
    if position.y >= grid.len() {
        return None;
    }
    let row = &grid[position.y];
    if position.x >= row.len() {
        return None;
    }
    Some(row[position.x])
}

// Define (x,y) position struct
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    // Move position, checking the boundary of the grid
    fn move_position(&self, direction: Direction, grid: &Vec<Vec<TileType>>) -> Option<Position> {
        match direction {
            Direction::N => {
                if self.y == 0 {
                    None
                } else {
                    Some(Position {
                        x: self.x,
                        y: self.y - 1,
                    })
                }
            }
            Direction::E => {
                if self.x == grid[self.y].len() - 1 {
                    None
                } else {
                    Some(Position {
                        x: self.x + 1,
                        y: self.y,
                    })
                }
            }
            Direction::S => {
                if self.y == grid.len() - 1 {
                    None
                } else {
                    Some(Position {
                        x: self.x,
                        y: self.y + 1,
                    })
                }
            }
            Direction::W => {
                if self.x == 0 {
                    None
                } else {
                    Some(Position {
                        x: self.x - 1,
                        y: self.y,
                    })
                }
            }
        }
    }
}

// NESW Direction
#[derive(Debug, Copy, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

// Define the TileType enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileType {
    NE,
    NW,
    SE,
    SW,
    EW,
    NS,
    GROUND,
    START,
}

// Parse input_test, returning Vec<Vec<TileType>>
fn parse_input(input: &str) -> Vec<Vec<TileType>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| TileType::from(c)).collect())
        .collect()
}


// Parse TileType from char
impl From<char> for TileType {

    fn from(c: char) -> Self {
        match c {
            '.' => TileType::GROUND,
            '|' => TileType::NS,
            '-' => TileType::EW,
            'J' => TileType::NW,
            '7' => TileType::SW,
            'L' => TileType::NE,
            'F' => TileType::SE,
            'S' => TileType::START,
            _ => panic!("Invalid tile type"),
        }
    }
}

impl TileType {
    fn connect(&self, position :Position, grid: &Vec<Vec<TileType>>) -> (Option<Position>, Option<Position>) {
    match self {
        TileType::NE => (position.move_position(Direction::N, grid), position.move_position(Direction::E, grid)),
        TileType::NW => (position.move_position(Direction::N, grid), position.move_position(Direction::W, grid)),
        TileType::SE => (position.move_position(Direction::S, grid), position.move_position(Direction::E, grid)),
        TileType::SW => (position.move_position(Direction::S, grid), position.move_position(Direction::W, grid)),
        TileType::EW => (position.move_position(Direction::E, grid), position.move_position(Direction::W, grid)),
        TileType::NS => (position.move_position(Direction::N, grid), position.move_position(Direction::S, grid)),
        TileType::GROUND => (None, None),
        _ => panic!("Invalid tile type"),
    }
    }
}


