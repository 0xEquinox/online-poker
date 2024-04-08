use crate::deck::{Card, Deck};
use crate::models::Player;
use crate::Lobbies;
use rocket::serde::Serialize;
use rocket::State;

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    pub settings: GameSettings,
    pub players: Vec<Player>,
    pub pot: u64,
    pub current_bet: u64,
    pub big_blind_pos: i32,
    pub small_blind_pos: i32,
    pub dealer_pos: i32,
    pub deck: Deck,
}

impl Game {
    pub fn new() -> Self {
        Self {
            settings: GameSettings::new(), // Default
            players: Vec::new(),
            pot: 0,
            current_bet: 0,
            big_blind_pos: 0,
            small_blind_pos: 0,
            dealer_pos: 0,
            deck: Deck::new(),
        }
    }
}

#[derive(Serialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct GameSettings {
    pub min_raise: u32, // As a percentage of the pot ie 0.5 is 50%
    pub starting_money: u64,
    pub small_blind: u32,
    pub big_blind: u32,
}

impl GameSettings {
    pub fn new() -> Self {
        Self {
            min_raise: 0,
            starting_money: 1000,
            small_blind: 2,
            big_blind: 4,
        }
    }
}
