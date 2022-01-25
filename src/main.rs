use rand::Rng;
use std::io;
use std::process;

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

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn deal_card(&mut self) -> Card {
        // loop until a card is returned
        loop {
            // generate a rand number for the card, simulating a shuffled deck
            let rand_num: u16 = rand::thread_rng().gen_range(0..52);

            // the card is still in the deck
            if self.cards[rand_num as usize].played == false {
                // clone the card
                let mut card = self.cards[rand_num as usize].clone();
                // set the card as played to true
                card.played = true;
                // set the card to played
                self.cards[rand_num as usize].played = true;
                // return the card to break the loop
                return card;
            }
        }
    }
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
    fn add_card(&mut self, card: Card, first: bool) {
        self.hand.push(card);

        self.score = self.calc_score();

        if !first {
            println!("\n=== Player {} ===\n", self.id);

            self.show_hand();
            self.show_score();
            self.check_busted();
        }
    }

    fn show_hand(&self) {
        for card in &self.hand {
            println!("Player {} is showing a {}", self.id, card.name);
        }
    }

    fn calc_score(&mut self) -> u8 {
        let mut score = 0;

        for card in &self.hand {
            if card.value != 11 {
                score += card.value;
            } else {
                let temp_score = score + 11;
                if temp_score > 21 {
                    score += 1;
                } else {
                    score += 11;
                }
            }
        }
        return score;
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
    fn add_card(&mut self, card: Card, first: bool) {
        self.hand.push(card);

        if !first {
            self.show_hand();
            self.score = self.calc_score();
            self.check_busted();

            println!("\n=== Dealer Debug ===\n");
            self.debug_show_score();
            self.debug_print_hand();
        }
    }

    fn debug_print_hand(&self) {
        for card in &self.hand {
            println!("{}", card);
        }
    }

    fn show_hand(&self) {
        // shows the last card in the dealers hand

        let card = self.get_last_card();
        let shown_card = card.unwrap();

        println!(
            "The dealer is showing a {}, and another card faced down.",
            shown_card.name
        );
    }

    fn get_last_card(&self) -> Option<&Card> {
        match self.hand.len() {
            0 => None,
            n => Some(&self.hand[n - 1]),
        }
    }

    fn calc_score(&mut self) -> u8 {
        let mut score = 0;

        for card in &self.hand {
            if card.value != 11 {
                score += card.value;
            } else {
                let temp_score = score + 11;
                if temp_score > 21 {
                    score += 1;
                } else {
                    score += 11;
                }
            }
        }
        return score;
    }

    fn check_busted(&mut self) {
        if self.score > 21 {
            self.busted = true;
            println!("Dealer is busted!");
        }
    }

    fn debug_show_score(&self) {
        println!("Dealer score: {}", self.score);
    }
}

// main function
fn main() {
    //

    println!("Welcome to Command Line Blackjack! Implemented in Rust.");
    println!("=======================================================");

    // let num_of_players = generate_player_count();

    // debug output
    // println!("There will be {} players\n", num_of_players);

    // creating the players, deck, and dealer objects

    // let players = generate_players(num_of_players);

    let player = generate_player();

    let deck = Deck {
        cards: generate_cards(),
    };

    let dealer = generate_dealer();

    play_game(player, dealer, deck);
}

fn generate_player_count() -> u16 {
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

        return players_num;
    };
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

// fn generate_players(number_of_players: u16) -> Vec<Player> {
//     let mut players: Vec<Player> = Vec::new();

//     for x in 1..(number_of_players + 1) {
//         let empty_hand: Vec<Card> = Vec::new();

//         players.push(Player {
//             id: x as u8,
//             bet: generate_bet(x),
//             score: 0,
//             busted: false,
//             hand: empty_hand,
//         })
//     }
//     return players;
// }

fn generate_player() -> Player {
    let empty_hand: Vec<Card> = Vec::new();

    let player = Player {
        id: 1,
        bet: generate_bet(1),
        score: 0,
        busted: false,
        hand: empty_hand,
    };
    return player;
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

fn play_game(mut player: Player, mut dealer: Dealer, mut deck: Deck) {
    println!("=======================================================");
    println!("Let the game begin\n");
    println!("The dealer is shuffling the cards...\n");

    // play first round function
    // deals two cards to the dealer and each player
    play_first_round(&mut player, &mut dealer, &mut deck);

    // while the dealer isnt busted or there are still players
    while !player.busted || !dealer.busted {
        play_round(&mut player, &mut dealer, &mut deck);
    }
}

fn play_first_round(player: &mut Player, dealer: &mut Dealer, deck: &mut Deck) {
    // deal each player and dealer two cards for the first round
    dealer.add_card(deck.deal_card(), true);
    dealer.add_card(deck.deal_card(), false);

    // deal the player two cards
    player.add_card(deck.deal_card(), true);
    player.add_card(deck.deal_card(), false);
}

fn play_round(player: &mut Player, dealer: &mut Dealer, deck: &mut Deck) {
    // if the player hasn't busted
    if !player.busted {
        // capturing the players move
        let player_move = player_round_input(player);

        // the player hits
        if player_move.contains("H") {
            println!("Player {} Hits!", player.id);
            hit(player, deck);
        }
        // the player stands
        else {
            println!("Player {} Stands!", player.id);

            // if the dealer's score is greater than the player's score
            if dealer.score > player.score {
                println!("Player {} has busted!", player.id);
                player.busted = true;
            }
        }
    }

    // if the dealer has lost
    if dealer.busted {
        println!("The Dealer is busted!");
        process::exit(0);
    }
    // if the dealer can hit
    else if !dealer.busted && dealer.score < 17 {
        // dealer.action();

        // generate number between 1 and 100
        let num: u16 = rand::thread_rng().gen_range(1..101);

        // if the dealer's score is less than 11 they will always hit
        if dealer.score <= 10 {
            dealer.add_card(deck.deal_card(), false);
        }
        // if the dealer's score is between 11 and 13 they have a 70% hit chance
        else if dealer.score > 10 && dealer.score < 14 {
            if num >= 30 {
                dealer.add_card(deck.deal_card(), false);
            }
        }
        // if the dealer's score is between 14 and 16 they have a 35% hit chance
        else if dealer.score >= 14 && dealer.score < 17 {
            if num >= 65 {
                dealer.add_card(deck.deal_card(), false);
            }
        }
    }

    if dealer.won {
        println!("The Dealer Wins");
        println!("The Dealers Score was: {}", dealer.score);
    }
}

fn player_round_input(player: &mut Player) -> String {
    loop {
        println!(
            "What will Player {}'s move be? (H) Hit, (S) Stand, (Q) Quit.",
            player.id
        );

        let mut p_m = String::new();

        io::stdin()
            .read_line(&mut p_m)
            .expect("Failed to read line");

        if p_m.to_uppercase().contains("Q") {
            process::exit(0);
        } else if p_m.to_uppercase().contains("H") || p_m.to_uppercase().contains("S") {
            return p_m.to_uppercase();
        }
    }
}

fn hit(player: &mut Player, deck: &mut Deck) {
    player.add_card(deck.deal_card(), false);
}

fn remove_player(players: &mut Vec<Player>, id: u8) -> Vec<Player> {
    let players = players
        .drain(..)
        .filter(|p| p.id != id)
        .collect::<Vec<Player>>();
    return players;
}
