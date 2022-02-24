use crate::card::Card;
use crate::deck::Deck;
use crate::score::Score;

#[derive(Debug)]
pub struct Player {
    pub hands: Vec<Card>,
    pub score: Score,
}

impl Player {
    pub fn new(deck: &mut Deck) -> Self {
        let mut hands = vec![];
        for _i in 0..2 {
            match deck.draw() {
                Some(card) => hands.push(card),
                None => panic!("Not found deck."),
            }
        }

        let cal_hand = hands.clone();
        Self {
            hands,
            score: Score::new(&cal_hand),
        }
    }
}
