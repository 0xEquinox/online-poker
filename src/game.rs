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
pub enum GameState {
    Deal,
    Preflop,
    Flop,
    Turn,
    River,
}

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
    pub current_players_turn: u32,
    pub state: GameState,
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
            current_players_turn: 0,
            state: GameState::Deal,
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
    Start,
    Fold,
    Check,
    Call,
    YourTurn,
    Anti(u64),
    Raise(u64),
    DealPlayer([Card; 2]),
    DealFlop([Card; 3]),
    DealTurn([Card; 1]),
    DealRiver([Card; 1]),
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
        Action::Fold => {
            fold(message.player_id, message.room_code, lobbies);
            next_turn(message.room_code, lobbies, queue);
        }
        Action::Check => {
            check();
            next_turn(message.room_code, lobbies, queue);
        }
        Action::Call => {
            call(message.player_id, message.room_code, lobbies);
            next_turn(message.room_code, lobbies, queue);
        }
        Action::Raise(amount) => {
            raise(amount, message.player_id, message.room_code, lobbies);
            next_turn(message.room_code, lobbies, queue);
        }
        Action::Start => start(message.room_code, lobbies, queue),
        _ => eprintln!("Invalid Move"),
    }

    let _ = queue.send(*message);
}

fn next_turn(room_code: i32, lobbies: &State<Lobbies>, queue: &State<Sender<Message>>) {
    println!("Moving to next player");
    let game = &lobbies.lobbies.get(&room_code).unwrap().game;

    let current_player_id = game.current_players_turn;
    let mut next_player_id: i32 = current_player_id as i32 + 1;

    // Check if it has to be wrapped around
    if current_player_id as usize == game.players.len() - 1 {
        next_player_id = 0;
    }

    // Let the next player know it is their turnYourTurn
    let next_player_message = Message {
        action: Action::YourTurn,
        room_code,
        player_id: next_player_id,
    };

    let _ = queue.send(next_player_message);
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

fn start(room_code: i32, lobbies: &State<Lobbies>, queue: &State<Sender<Message>>) {
    let mut binding = lobbies.lobbies.get_mut(&room_code).unwrap();
    let lobby = binding.value_mut();
    let game = &mut lobby.game;
    let players = &mut game.players;

    // Construct a message for the event queue, will need one message per player
    for player in &mut *players {
        let message = Message {
            action: Action::DealPlayer([game.deck.draw(), game.deck.draw()]),
            player_id: player.id, // -1 for dealer
            room_code,
        };

        let _ = queue.send(message);
    }

    // Also send out the anti message
    for player in players {
        let message = Message {
            action: Action::Anti(game.settings.small_blind as u64),
            player_id: player.id, // -1 for dealer
            room_code,
        };

        let _ = queue.send(message);
    }
    // Tell the first player it's their turn
    let message = Message {
        action: Action::YourTurn,
        player_id: 0,
        room_code,
    };

    let _ = queue.send(message);
}
