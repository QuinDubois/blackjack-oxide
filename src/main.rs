use std::io;
use std::cmp::Ordering;
use crate::card::Card;
use crate::deck::Deck;
use crate::hand::Hand;

mod card;
mod deck;
mod hand;
mod game;


fn main() {
    const DEALER: usize = 0; 
    
    println!("Welcome to Blackjack!");

    // Game Setup:
    // ask how many players 
    println!("How many players are playing? (min 1, max 5)");
    let mut player_count: isize;
    loop {
        let mut player_count_str: String = String::new();
        io::stdin().read_line(&mut player_count_str).expect("Failed to read line.");
        player_count = player_count_str.trim().parse().expect("Input is not an integer");

        if player_count > 0 && player_count <= 5 {
            break;
        } else {
            println!("Must be between 1 and 5 players.");
        }
    }

    // start game loop
    loop {
        println!("Starting a new round");
        // Make the deck
        let mut deck: Deck = Default::default();
        
        // add players and dealer to list
        let mut player_list: Vec<Hand> = Vec::new();
    
        for _player in 0..(player_count+1) {
            let hand: Hand = Default::default();
            player_list.push(hand);
        }

        // loop twice: deal a card to each player in the list
        for x in (0..(player_list.len()*2)).rev() {
            let mut player: usize = x;
            if x > player_list.len()-1 { player = player - player_list.len(); }
            let card: Card = deck.draw_card().unwrap();
            player_list[player].push_card(card);
        }

        // evaluate hands, keeping track of any natural 21s
        let mut natural_list: Vec<usize> = Vec::new();
        for player in (0..player_list.len()).rev() {
            match player_list[player].get_value().cmp(&21) {
                Ordering::Equal => {
                    natural_list.push(player);
                },
                Ordering::Greater => {
                    if player == DEALER {
                        println!("Dealer has busted! ... Wait that's not supposed to happen yet...");
                    } else {
                        println!("Player {} has busted! ... Wait that's not supposed to happen yet...", player);
                    }
                },
                Ordering::Less => (),
            }
        }

        // if any hands are natural, then game ends and those hands win
        if natural_list.len() > 0 {
            for hand in natural_list {
                if hand == DEALER {
                    println!("Dealer has a Natural!");
                } else {
                    println!("Player {} has a Natural!", hand);
                }
            }
            break;
        }

        // iterate through players, with the dealer last
        for player in (0..player_list.len()).rev() {
            if player == DEALER {
                loop {
                    let turn_end: bool = game::player_turn(true, &mut player_list, player, &mut deck);
                    if turn_end {
                        break;
                    }
                }
            } else {
                loop {
                    let turn_end: bool = game::player_turn(false, &mut player_list, player, &mut deck);
                    if turn_end {
                        break;
                    }
                }
            }
        }
    
        // highest hand value remaining wins or ties  
        // let mut winners_list: Vec<usize> = Vec::new();
        let mut max_score = 0;
        let mut winners_list: Vec<usize> = Vec::new();
        for player in (0..player_list.len()).rev() {
            let player_score = player_list[player].get_value();
            // TODO: accumulate winners

            if player_score > 21 {
                continue;
            } else {
                match player_score.cmp(&max_score) {
                    Ordering::Equal => {
                        winners_list.push(player);
                    },
                    Ordering::Greater => {
                        winners_list.clear();
                        max_score = player_score;
                        winners_list.push(player);
                    },
                    Ordering::Less => ()
                }
            }
        }

        for player in 0..winners_list.len() {
            if winners_list[player] == DEALER {
                println!("Dealer wins!");
            } else {
                println!("Player {} wins!", winners_list[player]);
            }
        }

        // TODO: Ask to play another round
        println!("Would you like to play again?");
        let play_again: char;
        let mut play_again_str: String = String::new();
        io::stdin().read_line(&mut play_again_str).expect("Failed to read line.");
        play_again = play_again_str.chars().nth(0).unwrap();

        if play_again == 'y' {
            println!("New round!");
        } else {
            break;
        }

    }
        
}

