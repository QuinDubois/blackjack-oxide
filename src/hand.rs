use crate::card::Card;

pub struct Hand {
    pub hand: Vec<Card>,
    value: u8,
}

impl Default for Hand {
    fn default() -> Self {
        Hand {hand: vec![], value: 0}
    }
}

impl Hand {
    fn eval_hand(&self) -> u8 {
        let mut hand_total = 0;
        let mut has_ace: bool = false;
    
        for card in &self.hand {
            hand_total += card.get_value();
            if card.get_value() == 1 {
                has_ace = true;
            }
        }
        // First ace in hand counts as 11 if it would not bust your hand
        if has_ace && hand_total < 12 {
            hand_total += 10;
        }
        hand_total
    }

    pub fn get_value(&self) -> u8 {
        return self.value
    }

    pub fn push_card(&mut self, card: Card) {
        self.hand.push(card);
        self.value = self.eval_hand();
    }

    pub fn to_string(&self, is_house: bool) -> String {
        let mut hand_str = String::from("");
        if is_house {
            // Dealer only reveals the first card they are dealt
            hand_str.push_str(&self.hand[0].to_string());
            hand_str
        } else {
            for card in &self.hand {
                hand_str.push_str(&card.to_string());
                hand_str.push_str(", ");
            }
            hand_str
        }
    }
}