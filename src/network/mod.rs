mod error;
mod message;

use super::channel;
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

/// Handles a TCP connection by attempting to establish a WebSocket connection.
async fn handle_connection(
    channels: Arc<Mutex<channel::SenderManager>>,
    raw_stream: TcpStream,
    addr: SocketAddr,
) -> NetworkResult<()> {
    println!("Incoming TCP connection from: {}", addr);
    // TODO: Include error message: "Error during the websocket handshake occurred."
    let ws_stream = tokio_tungstenite::accept_async(raw_stream).await?;
    println!("WebSocket connection established: {}", addr);

    // Insert the sender part of this channel into the channel manager.
    let (sender, receiver) = unbounded();
    channels.lock().unwrap().insert_client_sender(addr, sender);

    let (ws_out, ws_in) = ws_stream.split();

    // Handle each incoming WS message by sending a message on the sim channel.
    let ws_to_sim = ws_in.try_for_each(|ws_msg| {
        println!(
            "Received a message from {}: {}",
            addr,
            ws_msg.to_text().unwrap()
        );
        if let Ok(from_client) = message::IncomingMessage::try_from(ws_msg) {
            let message::IncomingMessage::RegisterInterest(region) = from_client;
            let channel_msg = channel::SimMsg::RegisterInterest(addr, region);
            channels.lock().unwrap().send_to_sim(channel_msg);
        }
        future::ok(())
    });

    // Forward messages recieved on this handler's channel to the outgoing WS stream.
    let channel_to_ws = receiver
        .map(|channel_msg| {
            let to_client = message::OutgoingMessage::from(channel_msg);
            Message::try_from(to_client)
        })
        .forward(ws_out.sink_err_into());

    pin_mut!(ws_to_sim, channel_to_ws);
    future::select(ws_to_sim, channel_to_ws).await;

    // Client is disconnected so remove it from the clients.
    println!("{} disconnected", &addr);
    channels.lock().unwrap().remove_client_sender(&addr);

    Ok(())
}
