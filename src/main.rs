use rand::Rng;
// use std::cmp::Ordering;
use std::io;

// struct House {
//     id: u8,
//     hand: Vec<Card>,
// }

struct Dealer {
    score: u8,
    busted: bool,
    won: bool,
    hand: Vec<Card>,
}
#[derive(Debug)]
struct Player {
    id: u8,
    bet: u32,
    score: u8,
    busted: bool,
    hand: Vec<Card>,
}

#[derive(Debug, Clone)]
struct Card {
    name: String,
    value: u8,
    played: bool,
    held_by: u8,
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

// implementing function on Player struct to print the players current hand
impl Player {
    fn show_hand(&self) {
        for card in &self.hand {
            println!("{}", card.name);
        }
    }

    fn calc_score(&mut self) {
        for card in &self.hand {
            if card.value != 11 {
                self.score += card.value;
            } else {
                let temp_score = self.score + 11;
                if temp_score > 21 {
                    self.score += 1;
                } else {
                    self.score += 11;
                }
            }
        }
    }

    fn check_busted(&mut self) {
        if self.score > 21 {
            self.busted = true;
            self.bet = 0;
            println!("Player {} is busted!", self.id);
        }
    }

    fn show_score(&self) {
        println!("Player {} score: {}", self.id, self.score);
    }
}

impl Dealer {
    fn show_hand(&self) {
        for card in &self.hand {
            println!("{}", card.name);
        }
    }

    fn calc_score(&mut self) {
        for card in &self.hand {
            if card.value != 11 {
                self.score += card.value;
            } else {
                let temp_score = self.score + 11;
                if temp_score > 21 {
                    self.score += 1;
                } else {
                    self.score += 11;
                }
            }
        }
    }

    fn check_busted(&mut self) {
        if self.score > 21 {
            self.busted = true;
            println!("Dealer is busted!");
        }
    }

    fn show_score(&self) {
        println!("Dealer score: {}", self.score);
    }
}

// main function
fn main() {
    //
    // const BUSTED: u8 = 22;

    let cards = generate_cards();

    let dealer = generate_dealer();

    // let house = House { id: 0, hand: cards };

    println!("Welcome to Command Line Blackjack! Implemented in Rust.");
    println!("=======================================================");

    // declaring player numbers variable as a new string data type
    let mut players_num = String::new();

    let num_of_players = loop {
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

        break players_num;
    };

    // debug output
    println!("There will be {} players\n", num_of_players);

    let players = generate_players(num_of_players);

    println!("{:?}", players);

    println!("=======================================================");
    println!("Let the game begin\n");
    println!("The dealer is shuffling the cards...\n");

    deal(dealer, players, cards);
}

fn generate_dealer() -> Dealer {
    let empty_hand: Vec<Card> = Vec::new();

    let dealer = Dealer {
        score: 0,
        busted: false,
        won: false,
        hand: empty_hand,
    };

    return dealer;
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
                    held_by: 0,
                });
            } else if x >= 8 && x < 12 {
                cards.push(Card {
                    name: String::from(&card_names[x + (13 * z)]),
                    value: 10,
                    played: PLAYED,
                    held_by: 0,
                });
            } else {
                cards.push(Card {
                    name: String::from(&card_names[x + (13 * z)]),
                    value: 11,
                    played: PLAYED,
                    held_by: 0,
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

fn generate_players(number_of_players: u16) -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();

    for x in 1..(number_of_players + 1) {
        let empty_hand: Vec<Card> = Vec::new();

        players.push(Player {
            id: x as u8,
            bet: generate_bet(x),
            score: 0,
            busted: false,
            hand: empty_hand,
        })
    }
    return players;
}

fn generate_bet(x: u16) -> u32 {
    let mut bet_input = String::new();

    let bet = loop {
        println!(
            "How much will Player {} bet? (No need to include the '$')",
            x
        );

        // capturing user input
        io::stdin()
            .read_line(&mut bet_input)
            .expect("Failed to read line");

        // converting user inputted string into immutable number
        let bet_input: u16 = match bet_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break bet_input;
    };

    return bet as u32;
}

fn deal(mut dealer: Dealer, players: Vec<Player>, mut cards: Vec<Card>) {
    for mut player in players {
        println!("===  Player {}  ===", player.id);
        player.hand = deal_cards(&mut cards);
        player.calc_score();
        player.show_hand();
        player.show_score();
        player.check_busted();
    }

    dealer.hand = deal_cards(&mut cards);
    println!("Dealer is showing a {}", dealer.hand[0].name);
    dealer.calc_score();
    dealer.show_score();
    dealer.check_busted();
}

fn deal_cards(cards: &mut Vec<Card>) -> Vec<Card> {
    let mut deal_count = 0;
    let mut hand: Vec<Card> = Vec::new();

    while deal_count < 2 {
        let rand_num = rand::thread_rng().gen_range(0..52);

        if cards[rand_num].played == false {
            hand.push(cards[rand_num].clone());
            cards[rand_num].played = true;
            deal_count += 1;
        }
    }

    return hand;
}

fn hit() {}

fn stand() {}
