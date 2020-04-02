mod grid;

use super::channel;
use super::geometry::BoundingBox;
use futures_channel::mpsc::unbounded;
use futures_util::{future, pin_mut, stream::StreamExt};
use grid::Grid;
use specs::prelude::*;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::time::delay_for;

type ClientInterests = HashMap<SocketAddr, BoundingBox>;

struct State<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    last_update: Instant, // TODO: Consider making an ECS resource if systems need to use it.
}

type Grass = u8;

struct GrowGrass;

impl<'a> System<'a> for GrowGrass {
    type SystemData = WriteExpect<'a, Grid<Grass>>;

    fn run(&mut self, data: Self::SystemData) {
        let mut grass_grid = data;
        for (x, y) in grass_grid.all_positions() {
            // TODO: This is gross. Figure out how to work with the borrow checker.
            let mut g = 0;
            {
                g = grass_grid.unsafe_at(x, y).to_owned();
            }
            grass_grid.set(x, y, (g + 1) % 5);
        }
    }
}

impl State<'_, '_> {
    pub fn new(x: u8, y: u8) -> Self {
        let mut world = World::new();
        // TODO: Register components.

        // Set up dispatcher and systems.
        let mut dispatcher = DispatcherBuilder::new()
            .with(GrowGrass, "grow_grass", &[])
            .build();
        dispatcher.setup(&mut world);

        // Load resources and entities.
        let grass_grid: Grid<Grass> = Grid::new(x, y, 0);
        world.insert(grass_grid);

        State {
            world: world,
            dispatcher: dispatcher,
            last_update: Instant::now(),
        }
    }
}

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
    let mut state = State::new(64, 64);
    let sim_loop = async {
        while let Ok(()) = loop_step(&mut state, client_interests.clone(), channels.clone()).await {
        }
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
async fn loop_step(
    state: &mut State<'_, '_>,
    client_interests: Arc<Mutex<ClientInterests>>,
    channels: Arc<Mutex<channel::Manager>>,
) -> Result<(), String> {
    // // Wait until a second has passed since last update.
    // let since_update = state.last_update.elapsed();
    // let wait_time: Duration = std::cmp::max(
    //     Duration::from_secs(0),
    //     Duration::from_secs(1) - since_update,
    // );

    //
    delay_for(Duration::from_millis(15)).await;
    if state.last_update.elapsed() >= Duration::from_secs(1) {
        state.dispatcher.dispatch(&state.world);
        state.world.maintain();
        state.last_update = Instant::now();
    }

    // Send updates to interested client handlers.
    let channels = channels.lock().unwrap();
    for (addr, region) in client_interests.lock().unwrap().iter() {
        let updates: Vec<channel::CellUpdate> = state
            .world
            .fetch::<Grid<Grass>>()
            .within(
                from_f32(region.x_min),
                from_f32(region.x_max),
                from_f32(region.y_min),
                from_f32(region.y_max),
            )
            .iter()
            .map(|(x, y, g)| channel::CellUpdate {
                x: i32::from(*x),
                y: i32::from(*y),
                grass: i32::from(*g.to_owned()),
            })
            .collect();
        let msg = channel::ClientHandlerMsg {
            cell_updates: updates,
        };
        channels.send_to_client_handler(&addr, msg);
    }

    Ok(())
}

fn from_f32(x: f32) -> u8 {
    if x < 0.0 {
        0
    } else if x > 255.0 {
        255
    } else {
        x as u8
    }
}
