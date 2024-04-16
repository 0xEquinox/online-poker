use crate::deck::{Card, Suit};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Player {
    pub money: u64,
    pub hand: (Card, Card),
    pub current_bet: u64,
    pub id: i32,
    pub is_in_round: bool,
}

impl Player {
    pub fn new(id: i32) -> Self {
        Self {
            money: 1000,
            hand: (Card {value: 1, suit: Suit::None}, Card{value: 1, suit: Suit::None}),
            current_bet: 0,
            id: id,
            is_in_round: false
        }
    }
}
