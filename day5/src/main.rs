use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let res = parse_file("input.txt");

    let seeds = res.0;
    let maps = res.1;

    // let expanded_maps = maps.iter()
    //     .map(|map| expand_map(&map.1))
    //     .collect::<Vec<HashMap<i64, i64>>>();

    let mut min_location = i64::MAX;

    // Iterate through all seeds, two at at a time
    let mut i = 0;
    while i <seeds.len()-1 {

        let seed_start = seeds[i];
        let seed_end = seed_start + seeds[i+1];
        i = i + 2;

        for seed in seed_start..seed_end {
            let mut current = seed;
    
            for map in &maps {
                current = get_value_raw(&map.1, current);
                //println!("{} : {} -> {}", step, previous, current)
            }
            // println!("-----------------------------------");
            // println!("{} -> {}", seed, current);
            // println!("-----------------------------------");
    
            if current < min_location {
                min_location = current;
            }

        }
    }

    println!("Min location: {}", min_location);

}    

fn expand_map(map: &Vec<(i64, i64, i64)>) -> HashMap<i64, i64> {

    let mut expanded_map = HashMap::new();

    for (destination, source, range) in map {
        for i in 0..*range {
            expanded_map.insert(*source+i, *destination + i);
        }
    }

    //println!("{:?}", expanded_map);

    expanded_map
}

fn get_value_raw(map: &Vec<(i64, i64, i64)>, key: i64) -> i64 {
    for (destination, source, range) in map {
        if key >= *source && key < *source + *range {
            return *destination + (key - *source);
        }
    }

    key
}

fn get_value(map: &HashMap<i64, i64>, key: i64) -> i64 {
    *map.get(&key).unwrap_or(&key)
}

fn parse_file(file_path: &str) -> (Vec<i64>, Vec<(String, Vec<(i64, i64, i64)>)>){
   
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap()   ;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();

    let seeds_line = lines.next().unwrap().unwrap();
    let seeds: Vec<i64> = seeds_line.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();

    let mut maps = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        if line.ends_with("map:") {
            let map_name = line.split_whitespace().next().unwrap().to_string();
            let mut map_values = Vec::new();

            while let Some(Ok(line)) = lines.next() {
                if line.is_empty() || line.ends_with("map:") {
                    break;
                }

                let values: Vec<i64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                map_values.push((values[0], values[1], values[2]));
            }

            maps.push((map_name, map_values));
        }
    }

    (seeds, maps)
}