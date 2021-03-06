//! To run:
//!
//!     cargo run 127.0.0.1:12345
//!
//! And then in another window run:
//!
//!     cargo run ws://127.0.0.1:12345/
//!
//! You can run the second command in multiple windows and then chat between the
//! two, seeing the messages from the other client as they're received. For all
//! connected clients they'll all join the same room and see everyone else's
//! messages.

mod geometry;
mod network;
mod simulation;

use futures_util::{future, pin_mut};
use network::channel;
use std::{
    env,
    io::Error as IoError,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let senders = Arc::new(Mutex::new(channel::SenderManager::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    // TODO: Include error message: "Failed to bind."
    let mut listener = try_socket?;
    println!("Listening on: {}", addr);

    // Run the connection handlers and simulation asynchronously.
    let handlers = network::accept_connections(&mut listener, senders.clone());
    let simulation = simulation::run(senders.clone());
    pin_mut!(handlers, simulation);
    future::select(handlers, simulation).await;

    Ok(())
}
