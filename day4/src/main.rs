fn main() {
    let input = include_str!("../input.txt");
    let cards = parse_input(input);

    //println!("{:?}", cards);
    let total_cards = cards.len();
    
    // Map of id to count
    let mut counts = std::collections::HashMap::new();

    // Add one count to counts for each card id
    for card in &cards {
        let count = counts.entry(card.id).or_insert(1);
    }

    for card in cards {
        let mut matches = 0;
        for winner in card.winners {
            if card.numbers.contains(&winner) {
                matches += 1;
            }
        }
        
        if matches == 0 {
            continue;
        }
        
        // Current count for this card
        let count = *counts.entry(card.id).or_insert(1);

        for i in 1..matches+1 {

            let new_id = card.id + i as i32;

            if new_id > (total_cards as i32)+1 {
                break;
            }
            // Increment the count for this card
            let new_count = counts.entry(new_id).or_insert(1);
            *new_count += count;
        }
    }

    
    let mut keys: Vec<_> = counts.keys().collect();
    // Sort the keys
    keys.sort();

    // Print the key-value pairs in sorted order of keys
    for key in keys {
        println!("{}: {}", key, counts[key]);
    }

    // Sum the counts
    let total: i32 = counts.values().sum();
    println!("Total: {}", total);

}

// debug
#[derive(Debug)]
struct Card {
    id: i32,
    winners: Vec<i32>,
    numbers: Vec<i32>,
}

fn parse_input(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    for line in input.lines() {

        println!("{}", line);

        let parts: Vec<&str> = line.split(':').collect();
        // id is chars 5 to the end of the first part
        let id_bits: Vec<&str> = parts[0].split_ascii_whitespace().collect();
        let id = id_bits[1].trim().parse().unwrap();

        // split part 2 by pipe character, winners is the whitespace separated first part
        let parts: Vec<&str> = parts[1].split('|').collect();

        let winners = parts[0].split_whitespace().map(|n| n.trim().parse().unwrap()).collect();
        let numbers = parts[1].split_whitespace().map(|n| n.trim().parse().unwrap()).collect();
        cards.push(Card { id, winners, numbers });
    }

    cards
}