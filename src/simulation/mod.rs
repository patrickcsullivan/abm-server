mod command_queue;
mod component;
mod frame;
mod grid;
mod snapshot;
mod state;
mod system;

use crate::network;
use crate::network::channel;
use frame::{DeltaFrame, Frame};
use futures_channel::mpsc::unbounded;
use futures_util::{future, pin_mut, stream::StreamExt};
use specs::prelude::*;
use state::State;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::time::delay_for;

/// Runs a single step of the simulation.
async fn step(
    state: &mut State<'_, '_>,
    inbox_buffer: Arc<Mutex<Vec<network::IncomingMessage>>>,
    senders: Arc<Mutex<channel::SenderManager>>,
) -> Result<(), String> {
    // Update the frame counter.
    if let Some(frame) = state.frame {
        // Wait until it's time for the next frame to start.
        let frame_duration = Duration::from_millis(Frame::DURATION_MILLIS);
        let duration_since_prev_ideal = Instant::now() - frame.ideal_start_time;
        if duration_since_prev_ideal < frame_duration {
            delay_for(frame_duration - duration_since_prev_ideal).await;
        }

        let next_frame = frame.next(Instant::now());
        let delta = next_frame.number - frame.number;
        state.frame = Some(next_frame);
        state.world.insert(DeltaFrame::new(delta));
    } else {
        state.frame = Some(Frame::new());
        state.world.insert(DeltaFrame::new(0));
    }

    {
        // Forward incoming messaging from the inbox buffer into
        let mut inbox_buffer = inbox_buffer.lock().unwrap();
        let mut inbox = state.world.fetch_mut::<Vec<network::IncomingMessage>>();
        while let Some(msg) = inbox_buffer.pop() {
            inbox.push(msg);
        }
    }

    // Execute a frame of the simulation.
    state.dispatcher.dispatch(&state.world);
    state.world.maintain();

    // Send all outgoing messages generated during the frame on the appropriate
    // client channels.
    let senders = senders.lock().unwrap();
    let mut outbox = state.world.fetch_mut::<Vec<network::OutgoingMessage>>();
    while let Some(msg) = outbox.pop() {
        senders.send_to_client(msg);
    }

    Ok(())
}

/// Push the incoming message into the inbox buffer.
async fn push_to_inbox_buffer(
    inbox_buffer: Arc<Mutex<Vec<network::IncomingMessage>>>,
    msg: network::IncomingMessage,
) {
    inbox_buffer.lock().unwrap().push(msg);
}

/// Runs the simulation.
pub async fn run(senders: Arc<Mutex<channel::SenderManager>>) -> Result<(), String> {
    // Insert the sender part of the simulation's channel into the sender
    // manager.
    let (sender, receiver) = unbounded();
    senders.lock().unwrap().insert_sim_sender(sender);

    // Push messages recieved on the simulation's channel into an inbox buffer.
    // The simulation loop will forward messages from the inbox buffer into the
    // inbox ECS resource between frames.
    let inbox_buffer = Arc::new(Mutex::new(vec![]));
    let handle_receiver = receiver.for_each(|msg| push_to_inbox_buffer(inbox_buffer.clone(), msg));

    // Run the simulation loop.
    let mut state = State::new();
    let sim_loop = async {
        while let Ok(()) = step(&mut state, inbox_buffer.clone(), senders.clone()).await {}
    };

    pin_mut!(handle_receiver, sim_loop);
    future::select(handle_receiver, sim_loop).await;
    Ok(())
}
