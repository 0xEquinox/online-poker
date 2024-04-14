use crate::deck::{Card, Deck};
use crate::models::Player;
use crate::Lobbies;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
enum Action {
    Fold,
    Check,
    Call,
    Raise { value: u64 },
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    action: Action,
    room_code: i32,
    player_id: i32,
}

#[post("/make_move", data = "<message>", format = "application/json")]
pub fn make_move(message: Json<Message>, lobbies: &State<Lobbies>) {}

// This function returns an infinite stream of events which will allow the clients to recieve updates in real time as things happen
// This does not allow for users to send updates to the server indefinetly, it's only a one way connection, which is perfect
//#[get("/events")]
//async fn events(lobbies: &State<Lobbies>, mut end: Shutdown) -> EventStream![] { //TODO Implement this
//}
