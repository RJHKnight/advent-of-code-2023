use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Draw {
    blue: i32,
    red: i32,
    green: i32,
}

#[derive(Debug)]
struct Game {
    id: i32,
    draws: Vec<Draw>,
}

fn parse_draw(draw_str: &str) -> Draw {
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;

    for color_str in draw_str.split(", ") {
        let parts: Vec<&str> = color_str.split(" ").collect();
        let count: i32 = parts[0].parse().unwrap();
        let color = parts[1];

        match color {
            "blue" => blue = count,
            "red" => red = count,
            "green" => green = count,
            _ => (),
        }
    }

    Draw { blue, red, green }
}

fn parse_game(game_str: &str) -> Game {
    let parts: Vec<&str> = game_str.split(": ").collect();
    let id_string = &parts[0][5..];
    let id: i32 = id_string.parse().unwrap();
    let draw_strs: Vec<&str> = parts[1].split("; ").collect();
    let draws: Vec<Draw> = draw_strs.into_iter().map(|s| parse_draw(s)).collect();

    Game { id, draws }
}

fn parse_games(file_path: &Path) -> io::Result<Vec<Game>> {
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);

    let mut games = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let game = parse_game(&line);
        games.push(game);
    }

    Ok(games)
}

fn main() {
    let games = parse_games(Path::new("input.txt")).unwrap();
    //println!("{:?}", games);

    let max_val = Draw{
        blue: 14,
        red: 12,
        green: 13,
    };
    
    let mut valid_games_sum = 0;
    let mut power_sum: i64 = 0;

    for game in games {

        let (mut max_red, mut max_blue, mut max_green) = (0,0,0);
        let mut valid_game = true;
        for draw in game.draws {
            if draw.blue > max_val.blue || draw.red > max_val.red || draw.green > max_val.green {
                valid_game = false;
            }

            if draw.blue > max_blue {
                max_blue = draw.blue;
            }
            if draw.red > max_red {
                max_red = draw.red;
            }
            if draw.green > max_green {
                max_green = draw.green;
            }
        }

        power_sum += max_red as i64 * max_blue as i64 * max_green as i64;

        if valid_game {
            valid_games_sum += game.id;
        }
    }

    println!("Sum of valid games: {}", valid_games_sum);
    println!("Sum of power: {}", power_sum);
}