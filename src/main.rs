#[macro_use]
extern crate rocket;
use dashmap::DashMap;
use rocket::{fs::relative, fs::FileServer, tokio::sync::broadcast::channel, Build, Rocket};

mod deck;
mod game;
mod lobby;
mod models;
mod websocket;

// The one important piece of state for this program is the hashmap of lobbies
struct Lobbies {
    lobbies: DashMap<i32, lobby::Lobby>,
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(Lobbies {
            lobbies: DashMap::new(),
        })
        .manage(channel::<game::Message>(1024).0)
        .mount(
            "/api/",
            routes![
                lobby::create_lobby,
                lobby::join_lobby,
                lobby::get_lobbies,
                game::make_move,
                websocket::events
            ],
        )
        .mount("/", FileServer::from(relative!("static")))
}
