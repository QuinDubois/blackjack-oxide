use crate::card::Card;
use crate::card::Suit;
use crate::card::Value;
use rand::Rng;

pub struct Deck {
    pub deck: Vec<Card>
}


impl Default for Deck {
    fn default() -> Self {
        let suits = [Suit::Diamond, Suit::Heart, Suit::Spade, Suit::Club];
        let values = [
            Value::Ace, 
            Value::Two, 
            Value::Three, 
            Value::Four, 
            Value::Five, 
            Value::Six, 
            Value::Seven, 
            Value::Eight, 
            Value::Nine, 
            Value::Ten, 
            Value::Jack, 
            Value::Queen, 
            Value::King
        ];

        let mut card_counter = 0;

        let mut deck: Vec<Card> = vec![Card{value: Value::Ace, suit: Suit::Spade}; 52];

        for suit in suits {
            for value in values {
                deck[card_counter] = Card {value, suit};
                card_counter += 1;
            }
        }

        Deck {deck}
    }
}

impl Deck {
    pub fn draw_card(&mut self) -> Option<Card> {
        let deck_size = self.deck.len();
        let card_idx = if deck_size > 0 {
            rand::thread_rng().gen_range(0..deck_size)
        } else {
            0
        };

        if deck_size > 0  {
            let drawn_card = self.deck.swap_remove(card_idx);
            Some(drawn_card)
        } else {
            None
        }
    }
}