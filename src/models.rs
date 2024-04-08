use rocket::serde::Serialize;
use crate::deck::{Card};

#[derive(Serialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Player {
    pub money: u64,
    pub hand: (Card, Card),
    pub current_bet: u64,
}
