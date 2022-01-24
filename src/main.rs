// use rand::Rng;
// use std::cmp::Ordering;
use std::io;

struct Dealer {
    face_up_card: Card,
    face_down_card: Card,
    score: u8,
    busted: bool,
}

struct Player {
    id: u8,
    bet: u32,
    score: u8,
    busted: bool,
}

#[derive(Debug)]
struct Card {
    name: String,
    value: u8,
    played: bool,
}

// implementing display for Card struct so Card struct can be printed to the terminal
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(Card name: {}, Card Value: {}, Card played: {})",
            self.name, self.value, self.played
        )
    }
}

// main function
fn main() {
    //
    // const BUSTED: u8 = 22;

    let cards = generate_cards();

    //////////////
    println!("Welcome to Command Line Blackjack! Implemented in Rust.");
    println!("=======================================================");

    //
    // declaring player numbers variable as a new string data type
    let mut players_num = String::new();

    loop {
        println!("How many players will there be?");

        // capturing user input
        io::stdin()
            .read_line(&mut players_num)
            .expect("Failed to read line");

        // converting user inputted string into immutable number
        let players_num: u16 = match players_num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    println!("There will be {} players", players_num);
    println!("{:?}", cards);
}

fn generate_cards() -> Vec<Card> {
    const PLAYED: bool = false;

    let card_types: [&str; 4] = ["Spades", "Hearts", "Diamonds", "Clubs"];

    let card_values: [&str; 13] = [
        "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th", "10th", "Jack", "Queen", "King",
        "Ace",
    ];

    let mut cards: Vec<Card> = Vec::new();

    let card_names: Vec<String> = generate_card_names(card_types, card_values);

    for z in 0..4 {
        for x in 0..13 {
            if x < 8 {
                cards.push(Card {
                    name: String::from(&card_names[x + (13 * z)]),
                    value: 2 + x as u8,
                    played: PLAYED,
                });
            } else if x >= 8 && x < 12 {
                cards.push(Card {
                    name: String::from(&card_names[x + (13 * z)]),
                    value: 10,
                    played: PLAYED,
                });
            } else {
                cards.push(Card {
                    name: String::from(&card_names[x + (13 * z)]),
                    value: 11,
                    played: PLAYED,
                });
            }
        }
    }

    return cards;
}

fn generate_card_names(card_types: [&str; 4], card_values: [&str; 13]) -> Vec<String> {
    let mut card_names: Vec<String> = Vec::new();

    for card_type in card_types {
        for value in card_values {
            let card = format!("{} of {}", value, card_type);
            card_names.push(String::from(card));
        }
    }

    return card_names;
}
