use strum::IntoEnumIterator;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::{Card, Suit, Number};

#[derive(Debug, Default)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut deck = Self::default();

        for suit in Suit::iter() {
            for num in Number::iter() {
                deck.cards.push(Card::new(suit.clone(), num.clone()));
            }
        }
        deck.shuffle();
        deck
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}