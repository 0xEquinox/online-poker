// Define the interface for joining and creating new lobbies

use crate::deck::{Card, Deck};
use crate::game::Game;
use crate::models::Player;
use crate::Lobbies;
use dashmap::DashMap;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Lobby {
    pub code: i32,
    pub game: Game,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct JoinData {
    code: i32,
    player: Player,
}

// First thing is to create new lobbies
#[get("/create_lobby")]
pub fn create_lobby(lobbies: &State<Lobbies>) -> Json<Lobby> {
    // Generating a code is easy just use the number of open lobbies
    let code = lobbies.lobbies.len() as i32;

    // Games will first be created with a default cosntructor and then users will have the option of changing the setttings later
    let game = Game::new();

    let lobby = Lobby { code, game };

    // Add lobby to the state
    lobbies.lobbies.insert(code, lobby.clone());

    return Json(lobby.clone());
}

#[post("/join_lobby", data = "<join_data>", format = "application/json")]
pub fn join_lobby(join_data: Json<JoinData>, lobbies: &State<Lobbies>) -> Json<Lobby> {
    let mut binding = lobbies.lobbies.get_mut(&join_data.code).unwrap();
    let lobby = binding.value_mut();

    lobby.game.players.push(join_data.player);

    return Json(lobby.clone());
}

#[get("/get_lobbies")]
pub fn get_lobbies(lobbies: &State<Lobbies>) -> Json<Vec<Lobby>> {
    let mut lobbies_vec: Vec<Lobby> = Vec::new();

    lobbies
        .lobbies
        .iter()
        .for_each(|lobby| lobbies_vec.push(lobby.value().clone()));

    return Json(lobbies_vec);
}
