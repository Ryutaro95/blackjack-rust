use crate::card::{Card, Suit, Number::*};

#[derive(Debug)]
pub struct Score {
    pub score: u8,
}

impl Score {
    pub fn new(hands: &Vec<Card>) -> Self {
        Self {
            score: Self::cal(&hands),
        }
    }

    pub fn cal(hands: &Vec<Card>) -> u8 {
        let mut score: u8 = 0;
        for card in hands.iter() {
            match card.number {
                Jack | Queen | King | Ten => score += 10 as u8,
                Two => score += 2 as u8,
                Three => score += 3 as u8,
                Four => score += 4 as u8,
                Five => score += 5 as u8,
                Six => score += 6 as u8,
                Seven => score += 7 as u8,
                Eight => score += 8 as u8,
                Nine => score += 9 as u8,
                Ace => score += 11 as u8,
            }
        }
        score
    }
}
