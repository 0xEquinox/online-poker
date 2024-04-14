#[macro_use]
extern crate rocket;
use dashmap::DashMap;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use std::collections::HashMap;

mod deck;
mod game;
mod lobby;
mod models;

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
        .mount(
            "/api/",
            routes![lobby::create_lobby, lobby::join_lobby, lobby::get_lobbies],
        )
}
