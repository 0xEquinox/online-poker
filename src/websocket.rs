use rocket::{Shutdown, State };
use rocket::tokio::sync::broadcast::{error::RecvError, channel, Sender};
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::select;
use crate::game::OutMessage;

#[get("/events")]
pub async fn events(queue: &State<Sender<OutMessage>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();

    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(n)) => continue
                },
                _ = &mut end => break
            };
            yield Event::json(&msg);
        }
    }
}
