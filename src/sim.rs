use super::channel;
use super::geometry::BoundingBox;
use futures_channel::mpsc::unbounded;
use futures_util::{future, pin_mut, stream::StreamExt};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::delay_for;

struct State {
    counter: i32,
}

type ClientInterests = HashMap<SocketAddr, BoundingBox>;

/// Runs the simulation.
pub async fn run(channels: Arc<Mutex<channel::Manager>>) -> Result<(), String> {
    // Insert the sender part of this channel into the channel manager.
    let (sender, receiver) = unbounded();
    channels.lock().unwrap().insert_sim(sender);

    // Keep track of clients' registered areas of interest.
    let client_interests = Arc::new(Mutex::new(HashMap::new()));

    // Handle messages recieved on the simulation's channel.
    let handle_receiver =
        receiver.for_each(|msg| handle_channel_msg(msg, client_interests.clone()));

    // Run the simulation loop.
    let mut state = State { counter: 0 };
    let sim_loop = async {
        while let Ok(()) = step(&mut state, client_interests.clone(), channels.clone()).await {}
    };

    pin_mut!(handle_receiver, sim_loop);
    future::select(handle_receiver, sim_loop).await;
    Ok(())
}

/// Handles the message into the simulation's channel by registering the
/// client's interest in an area of the simulation.
async fn handle_channel_msg(
    msg: channel::SimMsg,
    client_interests: Arc<Mutex<ClientInterests>>,
) -> () {
    let channel::SimMsg::RegisterInterest(addr, interest) = msg;
    client_interests.lock().unwrap().insert(addr, interest);
}

/// Runs a single step of the simulation.
async fn step(
    state: &mut State,
    client_interests: Arc<Mutex<ClientInterests>>,
    channels: Arc<Mutex<channel::Manager>>,
) -> Result<(), String> {
    // Mock some simulation logic.
    delay_for(Duration::from_millis(5000)).await;
    state.counter = state.counter + 1;
    println!("New state: {}", state.counter);

    // Send updates to interested client handlers.
    let channels = channels.lock().unwrap();
    let interests = client_interests.lock().unwrap();
    for (addr, _) in interests.iter() {
        // TODO: Use region to determine what updates to send.
        let msg = channel::ClientHandlerMsg {
            cell_updates: vec![],
        };
        channels.send_to_client_handler(&addr, msg);
    }

    Ok(())
}
