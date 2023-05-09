use crate::hand::Hand;

pub struct Player {
    pub hand_list: Vec<Hand>,
    pub number: u8,
}

impl Default for Player {
    fn default() -> self {
        Player { hand: vec![], number: 0 }
    }
}

impl Player {
    
}