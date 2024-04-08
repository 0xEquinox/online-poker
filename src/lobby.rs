// Define the interface for joining and creating new lobbies

use crate::deck::{Card, Deck};
use crate::game::Game;
use crate::Lobbies;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

use rocket::State;

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Lobby {
    pub code: i32,
    pub game: Game,
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
