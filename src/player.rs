use crate::card::{Card, Number::*};
use crate::deck::Deck;
use crate::score::Score;

pub trait Person {
    fn draw(&mut self, deck: &mut Deck);
    fn result_score(&self) -> Score;
}

#[derive(Debug, Default)]
pub struct Player {
    pub hands: Vec<Card>,
    pub score: Score,
}

impl Person for Player {
    fn draw(&mut self, deck: &mut Deck) {
        for _i in 0..2 {
            // It never fails.
            self.hands.push(deck.draw().unwrap());
        }
    }
    fn result_score(&self) -> Score {
        let mut score = 0 as u8;
        for card in self.hands.iter() {
            match card.number {
                Jack | Queen | King | Ten => score += 10 as u8,
                Ace => score += 11 as u8,
                i => score += i as u8,
            }
        }

        if score == 21 {
            return Score::BlackJack;
        } else if score > 21 {
            return Score::Burst;
        }

        Score::Point(score)
    }
}
