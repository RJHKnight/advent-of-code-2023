use std::collections::HashMap;


fn main() {

    // Use macro to read file to string
    let contents = include_str!("../input.txt");
    
    let contents = contents.split(",").collect::<Vec<&str>>();

    // Create a hashmap of new boxes with values 0..255
    let mut boxes = HashMap::new();
    for i in 0..256 {
        let this_box = Box::new();
        boxes.insert(i, this_box);
    }
    
    for i in 0..contents.len() {
        let this_entry = contents[i].to_string();
        let (label, is_add, lens) = handle_entry(this_entry);
        let this_string_val = get_string_val(&label);
        let this_box = boxes.get_mut(&this_string_val).unwrap();
        if is_add {
            this_box.add_lens(&label, lens);
        }
        else {
            this_box.remove_lens(&label);
        
        }
    }

    let mut working_val = 0;
    for i in 0..256 {
        let this_box = boxes.get(&i).unwrap();
        working_val += this_box.get_value(i);
    }

    println!("The value of the hash is: {}", working_val);
}

fn handle_entry(entry: String) -> (String, bool, i32) {
    // If entry contains '='
    if entry.contains("=") {
        // Split entry on '='
        let entry = entry.split("=").collect::<Vec<&str>>();
        // Get string value of entry[0]
        let this_string = entry[0].to_string();
        // Get string value of entry[1] and convert to i32
        let this_val = entry[1].parse::<i32>().unwrap();
        // Return (this_string, true, this_val)
        (this_string, true, this_val)
    } else {
        let entry = entry.split("-").collect::<Vec<&str>>();
        (entry[0].to_string(), false, 0)
    }
}


fn get_string_val(this_string: &String) -> i32 {

    let mut working_val = 0;

    for i in 0..this_string.len() {
        let this_char = this_string.chars().nth(i).unwrap();
        working_val = get_val(working_val, this_char);
    }

    working_val
}

fn get_val(working_val: i32, this_char: char) -> i32 {
    
    let mut this_val = working_val;

    // Get ASCII value of char
    let this_char_val = this_char as i32;

    this_val += this_char_val;

    this_val = this_val * 17;
    this_val = this_val % 256;

    this_val
}

// Define a box struct, containing a queue of string and a hash of string to i32
struct Box {
    queue: Vec<String>,
    hash: HashMap<String, i32>,
}

impl Box {

    fn new() -> Box {
        Box {
            queue: Vec::new(),
            hash: HashMap::new(),
        }
    }

    fn add_lens(&mut self, this_string: &String, lens: i32) {
        if !self.hash.contains_key(this_string) {
            self.queue.push(this_string.clone());
        }
        self.hash.insert(this_string.clone(), lens);
    }

    fn remove_lens(&mut self, this_string: &String) {
        self.hash.remove(this_string);
        self.queue.retain(|x| x != this_string);
    }

    fn get_value(&self, box_num: i32) -> i32 {
        let mut working_val = 0;
        for i in 0..self.queue.len() {
            let this_val = self.hash.get(&self.queue[i]).unwrap();
            working_val += (1+box_num) * this_val * (i as i32+1);
        }
        working_val
    }

}

// Test that get_string_val of HASH returns 52
#[test]
fn test_get_string_val() {
    let test_string = "HASH".to_string();
    let test_val = get_string_val(&test_string);
    assert_eq!(test_val, 52);
}