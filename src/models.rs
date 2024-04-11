use crate::deck::Card;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Player {
    pub money: u64,
    pub hand: (Card, Card),
    pub current_bet: u64,
}
