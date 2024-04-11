use rand::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Deck {
    cards: Vec<Card>,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Card {
    value: i32,
    suit: Suit,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
    None,
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

            for value in 1..=13 {
                cards.push(Card {
                    value,
                    suit: suit.clone(),
                });
            }
        }

        Self { cards }
    }

    fn dequeue(&mut self) -> Card {
        let mut rand = rand::thread_rng();

        let index = rand.gen_range(0..self.cards.len());
        let card: Card = *self.cards.get(index).unwrap();
        self.cards.remove(index);

        card
    }

    pub fn draw(&mut self) -> Card {
        self.dequeue()
    }

    pub fn draw_n(&mut self, n: i32) -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();

        for i in 0..n {
            cards.push(self.dequeue());
        }
        cards
    }

    pub fn shuffle(&mut self) {
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

            for value in 1..=13 {
                cards.push(Card {
                    value,
                    suit: suit.clone(),
                });
            }
        }

        self.cards = cards;
    }
}
