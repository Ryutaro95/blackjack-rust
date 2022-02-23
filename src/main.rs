mod card;
mod deck;

fn main() {
    println!("{:#?}", deck::Deck::new().cards.len());
}
