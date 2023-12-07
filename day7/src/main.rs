use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main()  {

    let mut hands = parse_file(Path::new("input.txt")).unwrap();

    // Sort hands by score
    hands.sort();

    // Print hands to a file
    let mut file = File::create("output.txt").unwrap();
    for hand in hands.iter() {
        let line = format!("{}{}{}{}{}\n", 
            hand.cards[0], hand.cards[1], 
            hand.cards[2], hand.cards[3], hand.cards[4]);
        file.write_all(line.as_bytes()).unwrap();
    }

    let mut score = 0;

    for i in 0..hands.len() {
        score += (i as i32 +1 ) * hands[i].bid;
    }

    println!("Score: {}", score);
}

// Copy trait
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Card {
    J,
    Value(u8),
    T,
    Q,
    K,
    A,
}

// fmt for card
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        let mut card = String::new();

        match self {
            Card::J => card = "J".to_string(),
            Card::Value(v) => card = v.to_string(),
            Card::T => card = "T".to_string(),
            Card::Q => card = "Q".to_string(),
            Card::K => card = "K".to_string(),
            Card::A => card = "A".to_string(),
        }

        write!(f, "{}", card)
    }
}

#[derive(Debug, Eq, Ord, Hash)]
struct Hand {
    bid: i32,
    cards: [Card; 5],
}

impl Hand {

    fn get_score(&self) -> u8 {
        let mut score = 0;

        if Hand::is_five_of_a_kind(self) {
            score = 10;
        } else if Hand::is_four_of_a_kind(self) {
            score = 9;
        } else if Hand::is_full_house(self) {
            score = 8;
        } else if Hand::is_three_of_a_kind(self) {
            score = 7;
        } else if Hand::is_two_pair(self) {
            score = 5;
        } else if Hand::is_one_pair(self) {
            score = 4;
        } else {
            score = 1;
        }

        
        score
    }

    fn get_text(&self) -> String {
        
        let score = self.get_score();
        let names: HashMap<u8, &str> = HashMap
            ::from_iter(vec![
                (10, "Five"),
                (9, "Four"),
                (8, "FullHouse"),
                (7, "Three"),
                (5, "TwoPair"),
                (4, "OnePair"),
                (1, "HighCard")
            ]);

        names.get(&score).unwrap().to_string()
    }

    fn is_five_of_a_kind(hand :&Hand) -> bool {
        
        let mut is_five = false;
        let mut count = HashMap::new();
        
        for card in hand.cards.iter() {
            let count = count.entry(card).or_insert(0);
            *count += 1;
        }

        let num_wild = count.get(&Card::J).unwrap_or(&0);
                 // Get sorted values excluding J keys
        let values = count
            .iter()
            .filter(|(k, _)| ***k != Card::J)
            .map(|(_, v)| *v)
            .collect::<Vec<_>>();
     
        let max_value = values.iter().max();
        // unwrap or default to 0
        let max_value = match max_value {
            Some(v) => *v,
            None => 0,
        };

        if max_value + *num_wild == 5 || *num_wild == 5{
            is_five = true;
        }

        is_five
    }

    fn is_four_of_a_kind(hand :&Hand) -> bool {
        let mut is_four = false;
        let mut count = HashMap::new();
        
        for card in hand.cards.iter() {
            let count = count.entry(card).or_insert(0);
            *count += 1;
        }

        let num_wild = count.get(&Card::J).unwrap_or(&0);

         // Get sorted values excluding J keys
         let values = count
            .iter()
            .filter(|(k, _)| ***k != Card::J)
            .map(|(_, v)| *v)
            .collect::<Vec<_>>();

        let max_value = values.iter().max().unwrap();

        if max_value + *num_wild == 4 || *num_wild == 4{
            is_four = true;
        }

        is_four
    }

    fn is_three_of_a_kind(hand :&Hand) -> bool {
        let mut is_three = false;
        let mut count = HashMap::new();
        
        for card in hand.cards.iter() {
            let count = count.entry(card).or_insert(0);
            *count += 1;
        }

        let num_wild = count.get(&Card::J).unwrap_or(&0);
        // Get sorted values excluding J keys
        let values = count
            .iter()
            .filter(|(k, _)| ***k != Card::J)
            .map(|(_, v)| *v)
            .collect::<Vec<_>>();

        let max_value = values.iter().max().unwrap();

        if max_value + *num_wild == 3 || *num_wild == 3{
            is_three = true;
        }

        is_three
    }

    fn is_two_pair(hand :&Hand) -> bool {
        let mut is_two_pair = false;
        let mut count = HashMap::new();
        
        for card in hand.cards.iter() {
            let count = count.entry(card).or_insert(0);
            *count += 1;
        }

        let num_wild = count.get(&Card::J).unwrap_or(&0);

        // Get sorted values excluding J keys
        let mut values = count
            .iter()
            .filter(|(k, _)| ***k != Card::J)
            .map(|(_, v)| *v)
            .collect::<Vec<_>>();

        values.sort();

        let max_value = values[values.len()-1];
        let next_max_value = values[values.len()-2];

        if *num_wild == 1 && max_value == 2 && next_max_value == 1 ||
            *num_wild == 2 && (max_value  == 2)  ||
            max_value == 2 && next_max_value == 2 ||
            *num_wild == 2 && max_value == 1 && next_max_value == 1 ||
            *num_wild == 3 && max_value == 1 {
            is_two_pair = true;
        }

        is_two_pair
    }

    fn is_one_pair(hand :&Hand) -> bool {
        let mut is_pair = false;
        let mut count = HashMap::new();
        
        for card in hand.cards.iter() {
            let count = count.entry(card).or_insert(0);
            *count += 1;
        }

        let num_wild = count.get(&Card::J).unwrap_or(&0);

        let max_value = count.values().max().unwrap();

        if *max_value == 2 || *num_wild > 0{
            is_pair = true;
        }

        is_pair
    }

    fn is_full_house(hand :&Hand) -> bool {
        
        let mut is_full = false;
        let mut count = HashMap::new();
        
        for card in hand.cards.iter() {
            let count = count.entry(card).or_insert(0);
            *count += 1;
        }

        let num_wild = count.get(&Card::J).unwrap_or(&0);
        
        // Get sorted values excluding J keys
        let mut values = count
            .iter()
            .filter(|(k, _)| ***k != Card::J)
            .map(|(_, v)| *v)
            .collect::<Vec<_>>();

        values.sort();


        let max_value = values[values.len()-1];
        let mut next_max_value = 0;
        
        if values.len() >= 2 {
            next_max_value = values[values.len()-2];
        }

        if  max_value == 3 && next_max_value == 2 ||
            *num_wild == 3 && max_value == 2 ||
            *num_wild == 2 && max_value == 3  ||
            *num_wild == 2 && max_value == 2 && next_max_value == 1 ||
            *num_wild == 1 && max_value == 2 && next_max_value == 2 {
            is_full = true;
        }

        is_full
    }
}


impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        // Define your own logic for comparing hands for ordering
        let hand_score = self.get_score().eq(&other.get_score());

        if hand_score == true {

            for i in 0..5 {
                let card_score = self.cards[i].eq(&other.cards[i]);
                if !card_score {
                    return false;
                }
            }

            return true;
        } else {
            return false;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        let hand_score = self.get_score().partial_cmp(&other.get_score());

        if hand_score == Some(Ordering::Equal) {

            for i in 0..5 {
                let card_score = self.cards[i].partial_cmp(&other.cards[i]);
                if card_score != Some(Ordering::Equal) {
                    return card_score;
                }
            }

            return Some(Ordering::Equal);
        } else {
            return hand_score;
        }
    }

}

fn parse_line(line: &str) -> Hand {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let cards = parts[0].chars().map(|c| match c {
        '1'..='9' => Card::Value(c.to_digit(10).unwrap() as u8),
        'T' => Card::T,
        'J' => Card::J,
        'Q' => Card::Q,
        'K' => Card::K,
        'A' => Card::A,
        _ => panic!("Invalid card character {}", c),
    }).collect::<Vec<Card>>();
    let number = parts[1].parse::<i32>().unwrap();
    let hand = Hand {bid: number, cards: [cards[0], cards[1], cards[2], cards[3], cards[4]] };
    hand
}

fn parse_file(file_path: &Path) -> io::Result<Vec<Hand>> {
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);
    let mut hands = Vec::new();

    for line in reader.lines() {
        let line = line?;
        hands.push(parse_line(&line));
    }
    Ok(hands)
}

// Test that a hand of 1 1 2 3 J is two pair
#[test]
fn test_two_pair() {
    let hand = Hand {bid: 1, cards: [Card::Value(1), Card::Value(1), Card::Value(2), Card::Value(3), Card::J] };
    assert_eq!(Hand::is_two_pair(&hand), true);
}

// test that a hand of 1 2 3 J J is two pair
#[test]
fn test_two_pair2() {
    let hand = Hand {bid: 1, cards: [Card::Value(1), Card::Value(2), Card::Value(3), Card::J, Card::J] };
    assert_eq!(Hand::is_two_pair(&hand), true);
}

// test that a hand of 1 1 2 3 4 is not two pair
#[test]
fn test_two_pair3() {
    let hand = Hand {bid: 1, cards: [Card::Value(1), Card::Value(1), Card::Value(2), Card::Value(3), Card::Value(4)] };
    assert_eq!(Hand::is_two_pair(&hand), false);
}

// test that the following hands are full house
// 1 1 1 2 2
// 1 1 2 2 J
// 1 1 2 J J
// 1 1 J J J
#[test]
fn test_full_house() {
    let hand = Hand {bid: 1, cards: [Card::Value(1), Card::Value(1), Card::Value(1), Card::Value(2), Card::Value(2)] };
    assert_eq!(Hand::is_full_house(&hand), true);

    let hand = Hand {bid: 1, cards: [Card::Value(1), Card::Value(1), Card::Value(2), Card::Value(2), Card::J] };
    assert_eq!(Hand::is_full_house(&hand), true);

    let hand = Hand {bid: 1, cards: [Card::Value(1), Card::Value(1), Card::Value(2), Card::J, Card::J] };
    assert_eq!(Hand::is_full_house(&hand), true);

    let hand = Hand {bid: 1, cards: [Card::Value(1), Card::Value(1), Card::J, Card::J, Card::J] };
    assert_eq!(Hand::is_full_house(&hand), true);
}

// test that JJAJT is four of a kind
#[test]
fn test_four_of_a_kind() {
    let hand = Hand {bid: 1, cards: [Card::J, Card::J, Card::A, Card::J, Card::T] };
    assert_eq!(Hand::is_four_of_a_kind(&hand), true);
}

// test that 2JJ2J is a five
#[test]
fn test_full_house2() {
    let hand = Hand {bid: 1, cards: [Card::Value(2), Card::J, Card::J, Card::Value(2), Card::J] };
    assert_eq!(Hand::is_five_of_a_kind(&hand), true);
}

// test that the hand 1 1 J J 2 is greater than J J 1 1 2
#[test]
fn test_greater_than() {
    let hand1 = Hand {bid: 1, cards: [Card::Value(1), Card::Value(1), Card::J, Card::J, Card::Value(2)] };
    let hand2 = Hand {bid: 1, cards: [Card::J, Card::J, Card::Value(1), Card::Value(1), Card::Value(2)] };
    assert_eq!(hand1 > hand2, true);
}