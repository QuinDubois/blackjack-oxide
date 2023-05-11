
use std::io;
use std::cmp::Ordering;
use crate::card::Card;
use crate::deck::Deck;
use crate::hand::Hand;

pub fn get_user_move() -> char{
    let mut choice = String::new();
    
    println!("Do you want to Hit or Stay? h/s");
    io::stdin().read_line(&mut choice).unwrap();
    return choice.chars().nth(0).unwrap()
}

pub fn player_turn(is_dealer: bool, player_vec: &mut Vec<Hand>, player: usize, deck: &mut Deck) -> bool {
    let mut turn_end: bool = false;
    let mut hand_value: u8 = player_vec[player].get_value();
    // TODO: dealer turn:
    if is_dealer {
        // if dealer hand <= 16, deal a card and restart dealer turn
        if hand_value <= 16 {
            println!("Dealer hits.");
            let card: Card = deck.draw_card().unwrap();
            player_vec[player].push_card(card);
        // if dealer hand > 16 and <= 21, dealer stays
        } else if hand_value > 16 && hand_value <= 21 {
            println!("Dealer stays.");
            turn_end = true;
        // if dealer hand > 21, bust dealer and remove from player_list
        }  else {
            println!("Dealer busts!");
            turn_end = true;
        }
    } else {
        // TODO: player turns:
        // show hand
        println!("Player {}'s hand: {}", player, player_vec[player].to_string(false));
        // ask if player wants to hit or stay
        let player_move: char = get_user_move();
        match player_move {
            'h' => {
                // if player hits, deal a card
                let card: Card = deck.draw_card().unwrap();
                player_vec[player].push_card(card);
                hand_value = player_vec[player].get_value();

                match hand_value.cmp(&21){
                    // if hand value is < 21, restart player turn
                    Ordering::Less => (),
                    // if hand value is = 21, move on to next player
                    Ordering::Equal => {
                        println!("Blackjack!");
                        turn_end = true;
                    },
                    // if hand value is > 21, bust player and remove from player_list
                    Ordering::Greater => {
                        println!("Busted!");
                        turn_end = true;
                    },
                }
            },
            // if player stays, move on to next player
            's' => {
                turn_end = true;
            },
            _ => {println!("Not a valid move")},
        }
    }

    return turn_end
}