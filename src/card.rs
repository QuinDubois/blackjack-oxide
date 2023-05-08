#[derive(Copy, Clone)]
pub enum Suit {
    Spade,
    Diamond,
    Club,
    Heart,
}

#[derive(Copy, Clone)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl Card {
    pub fn to_string(&self) -> String {
        let name = match &self.suit {
            Suit::Spade => String::from("Spades"),
            Suit::Diamond => String::from("Diamonds"),
            Suit::Club => String::from("Clubs"),
            Suit::Heart => String::from("Hearts"),
        };

        let value = match &self.value {
            Value::Ace => String::from("Ace"),
            Value::Two => String::from("Two"),
            Value::Three => String::from("Three"),
            Value::Four => String::from("Four"),
            Value::Five => String::from("Five"),
            Value::Six => String::from("Six"),
            Value::Seven => String::from("Seven"),
            Value::Eight => String::from("Eight"),
            Value::Nine => String::from("Nine"),
            Value::Ten => String::from("Ten"),
            Value::Jack => String::from("Jack"),
            Value::Queen => String::from("Queen"),
            Value::King => String::from("King"),
        };

        let mut name_str = String::from("");

        name_str.push_str(&value);
        name_str.push_str(" of ");
        name_str.push_str(&name);

        name_str
    }

    pub fn get_value(self) -> u8 {
        let value = match &self.value {
            Value::Ace => 1,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            _ => 10,
        };
        value
    }
}