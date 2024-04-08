use rocket::serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Deck {
    cards: Vec<Card>,
}

#[derive(Serialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Card {
    value: i32,
    suit: Suit,
}

#[derive(Serialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Deck {
    pub fn new() -> Self {
        // Populate the deck of cards
        let mut cards: Vec<Card> = Vec::new();
        for i in 0..4 {
            let suit = match i {
                0 => Suit::Heart,
                1 => Suit::Club,
                2 => Suit::Diamond,
                3 => Suit::Spade,
                _ => Suit::Heart,
            };

            for value in 0..13 {
                cards.push(Card {
                    value,
                    suit: suit.clone(),
                });
            }
        }

        Self { cards }
    }
}
