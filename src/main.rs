mod card;
mod deck;
mod player;
mod score;
mod dealer;

use deck::Deck;
use player::{Player, Person};
use dealer::Dealer;


fn main() {
    let (mut player, mut dealer) = start();
    println!("{:#?}", player);
    println!("{:#?}", dealer);
}

fn start() -> (Player, Dealer) {
    let mut deck = Deck::new();
    let mut player = Player::default();
    let mut dealer = Dealer::default();

    player.draw(&mut deck);
    player.score = player.result_score();

    dealer.draw(&mut deck);
    dealer.score = dealer.result_score();
    (player, dealer) 
}