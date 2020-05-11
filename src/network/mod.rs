pub mod channel;
mod error;
mod message;

pub use message::{IncomingMessage, OutgoingMessage};

use error::NetworkResult;
use futures::sink::SinkExt;
use futures_channel::mpsc::unbounded;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use std::{
    convert::TryFrom,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

/// Handles a TCP connection by attempting to establish a WebSocket connection.
async fn handle_connection(
    channels: Arc<Mutex<channel::SenderManager>>,
    raw_stream: TcpStream,
    addr: SocketAddr,
) -> NetworkResult<()> {
    println!("Incoming TCP connection from: {}", addr);
    // TODO: Include error message: "Error during the websocket handshake
    // occurred."
    let ws_stream = tokio_tungstenite::accept_async(raw_stream).await?;
    println!("WebSocket connection established: {}", addr);

    // Insert the sender part of this channel into the channel manager.
    let (sender, receiver) = unbounded();
    channels.lock().unwrap().insert_client_sender(addr, sender);

    let (ws_out, ws_in) = ws_stream.split();

    // Handle each incoming WS message by sending a message on the sim channel.
    let handle_incoming_messages = ws_in.try_for_each(|ws_msg| {
        println!(
            "Received a message from {}: {}",
            addr,
            ws_msg.to_text().unwrap()
        );
        if let Ok(incoming_msg) = message::IncomingMessage::try_new(addr, ws_msg) {
            channels.lock().unwrap().send_to_sim(incoming_msg);
        }
        future::ok(())
    });

    // Forward messages recieved on this handler's channel to the outgoing WS
    // stream.
    let handle_outgoing_messages = receiver
        .map(Message::try_from)
        .forward(ws_out.sink_err_into());

    pin_mut!(handle_incoming_messages, handle_outgoing_messages);
    future::select(handle_incoming_messages, handle_outgoing_messages).await;

    // Client is disconnected so remove it from the clients.
    println!("{} disconnected", &addr);
    channels.lock().unwrap().remove_client_sender(&addr);

    Ok(())
}

/// Returns a future thant accepts each new connection from the TCP listener in
/// a separate task.
pub async fn accept_connections(
    listener: &mut TcpListener,
    channels: Arc<Mutex<channel::SenderManager>>,
) {
    while let Ok((stream, addr)) = listener.accept().await {
        // Spawn separate task for handing each connection.
        tokio::spawn(handle_connection(channels.clone(), stream, addr));
    }
}
