#[derive(Debug)]
pub enum Score {
    BlackJack,
    Burst,
    Point(u8),
}

impl Default for Score {
    fn default() -> Self { Score::Point(0) }
}
