use crate::deck::{Card, Deck};
use crate::models::Player;
use crate::Lobbies;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::State;
use ws::WebSocket;

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
pub enum Action {
    Fold,
    Check,
    Call,
    Raise(u64),
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub action: Action,
    pub room_code: i32,
    pub player_id: i32,
}

#[post("/make_move", data = "<message>", format = "application/json")]
pub fn make_move(message: Json<Message>, lobbies: &State<Lobbies>, queue: &State<Sender<Message>>) {
    match message.action {
        Action::Fold => fold(message.player_id, message.room_code, lobbies),
        Action::Check => check(),
        Action::Call => call(message.player_id, message.room_code, lobbies),
        Action::Raise(amount) => raise(amount, message.player_id, message.room_code, lobbies),
    }

    let _ = queue.send(*message);
}

fn check() {
    println!("Player checks");
}

fn fold(player_id: i32, room_code: i32, lobbies: &State<Lobbies>) {
    println!("Player Folds");
    let mut binding = lobbies.lobbies.get_mut(&room_code).unwrap();
    let lobby = binding.value_mut();
    let game = &mut lobby.game;
    let players = &mut game.players;

    // Find the correct player
    let mut player = players
        .iter_mut()
        .filter(|player| player.id == player_id)
        .collect::<Vec<&mut Player>>();

    let player = &mut player[0];

    player.is_in_round = false;
}

fn call(player_id: i32, room_code: i32, lobbies: &State<Lobbies>) {
    println!("Player calls");
    // Find the correct lobby
    let mut binding = lobbies.lobbies.get_mut(&room_code).unwrap();
    let lobby = binding.value_mut();
    let game = &mut lobby.game;
    let players = &mut game.players;

    // Find the correct player
    let mut player = players
        .iter_mut()
        .filter(|player| player.id == player_id)
        .collect::<Vec<&mut Player>>();

    let player = &mut player[0];

    // Adjust the player's money
    let diff = game.current_bet - player.current_bet;

    player.current_bet = game.current_bet;
    player.money -= diff;
    game.pot += diff;
}

fn raise(amount: u64, player_id: i32, room_code: i32, lobbies: &State<Lobbies>) {
    println!("Player raises");
    let mut binding = lobbies.lobbies.get_mut(&room_code).unwrap();
    let lobby = binding.value_mut();
    let game = &mut lobby.game;
    let players = &mut game.players;

    // Find the correct player
    let mut player = players
        .iter_mut()
        .filter(|player| player.id == player_id)
        .collect::<Vec<&mut Player>>();

    let player = &mut player[0];

    // Set the game bet to the player's bet
    player.current_bet += amount;
    game.pot += amount;
    game.current_bet = player.current_bet;
}
