use strum_macros::EnumIter;

#[derive(Debug, Clone)]
pub struct Card {
    pub suit: Suit,
    pub number: Number,
}

#[derive(Debug, Clone, EnumIter)]
pub enum Suit {
    Diamond,
    Heart,
    Spade,
    Club,
}

#[derive(Debug, Clone, EnumIter, Copy)]
pub enum Number {
    Ace,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack,
    Queen,
    King,
}

impl Card {
    pub fn new(suit: Suit, number: Number) -> Self {
        Self { suit, number }
    }
}
