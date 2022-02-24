mod card;
mod deck;
mod player;
mod score;

use deck::Deck;
use player::Player;
use card::Number::*;


fn main() {
    let mut deck = Deck::new();


    println!("ドロー前のデッキ枚数: {}", deck.cards.len());
    let mut palyer = Player::new(&mut deck);
    println!("ドロー前のデッキ枚数: {}", deck.cards.len());
    println!("{:#?}", palyer.hands);


    println!("--------------------------- Player ---------------------------");
    print!("手札: | ");
    for card in palyer.hands.iter() {
        print!("{:?}: ", card.suit);
        match card.number {
            Ace => {
                print!("{} | ", 11u8);
            }
            Ten | Jack | Queen | King => print!("{} | ", 10u8),
            _ => print!("{} | ", card.number as u8),
        }
    }

    println!("あなたのスコアは{:?}", palyer.score);

    // For display confirmation
    todo!("hoge");
}
