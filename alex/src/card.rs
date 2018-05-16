#[derive(Copy, Clone)]
pub enum Suit {
    hearts,
    spades,
    diamonds,
    clubs,
}

#[derive(Copy, Clone, new)]
pub struct Card {
    tipo: Suit,
}
