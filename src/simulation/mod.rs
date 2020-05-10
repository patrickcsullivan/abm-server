mod command_queue;
mod component;
mod frame;
mod grid;
mod snapshot;
mod system;

use super::channel;
use super::geometry::BoundingBox;
use command_queue::{CreateSheepCommand, CreateSheepCommandQueue};
use frame::{DeltaFrame, Frame};
use futures_channel::mpsc::unbounded;
use futures_util::{future, pin_mut, stream::StreamExt};
use specs::prelude::*;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::time::delay_for;

struct State<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    frame: Option<Frame>,
}

impl State<'_, '_> {
    pub fn new() -> Self {
        // Register components.
        let mut world = World::new();
        world.register::<component::Position>();
        world.register::<component::Heading>();
        world.register::<component::Velocity>();
        world.register::<component::SheepBehaviorState>();

        // Set up dispatcher and systems.
        let mut dispatcher = DispatcherBuilder::new().build();
        dispatcher.setup(&mut world);

        // Initialize data.
        State::initialize_cmd_queue(&mut world);
        State::initialize_snapshots(&mut world);

        // Set up dispatcher and systems.
        let mut dispatcher = DispatcherBuilder::new()
            .with(system::DebugLogSystem, "debug_log", &[])
            .with(
                system::ResetAllSheepSnapshotSystem,
                "reset_all_sheep_snapshot",
                &["debug_log"],
            )
            .with(
                system::AllSheepSnapshotSystem,
                "all_sheep_snapshot",
                &["reset_all_sheep_snapshot"],
            )
            .with(
                system::SheepHeadingSystem,
                "sheep_heading",
                &["all_sheep_snapshot"],
            )
            .with(
                system::SheepVelocitySystem,
                "sheep_velocity",
                &["sheep_heading"],
            )
            .with(system::PositionSystem, "position", &["sheep_velocity"])
            .with(system::CreateCommandSystem, "create_command", &["position"])
            .build();
        dispatcher.setup(&mut world);

        State {
            world,
            dispatcher,
            frame: None,
        }
    }

    fn initialize_cmd_queue(world: &mut World) {
        let mut create_cmds = CreateSheepCommandQueue::new();
        for x in 1..=5 {
            for y in 1..=5 {
                create_cmds.push(CreateSheepCommand {
                    position: component::Position::new((x * 3) as f32, (y * 3) as f32),
                    heading: component::Heading::new(0.0),
                    velocity: component::Velocity::new(0.0, 0.0),
                    behavior: component::SheepBehaviorState::new(component::SheepBehavior::Walking),
                })
            }
        }
        world.insert(create_cmds);
    }

    fn initialize_snapshots(world: &mut World) {
        world.insert(snapshot::AllSheepSnapshot::new(16, 16));
        world.insert(snapshot::RunningSheepSnapshot::new(16, 16));
    }
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
    let mut state = State::new();
    let sim_loop = async {
        while let Ok(()) = step(&mut state, client_interests.clone(), channels.clone()).await {}
    };

    pin_mut!(handle_receiver, sim_loop);
    future::select(handle_receiver, sim_loop).await;
    Ok(())
}

/// Handles the incoming message on the simulation channel by registering the
/// client's interest in an area of the simulation.
async fn handle_channel_msg(msg: channel::SimMsg, client_interests: Arc<Mutex<ClientInterests>>) {
    let channel::SimMsg::RegisterInterest(addr, interest) = msg;
    client_interests.lock().unwrap().insert(addr, interest);
}

/// Runs a single step of the simulation.
async fn step(
    state: &mut State<'_, '_>,
    client_interests: Arc<Mutex<ClientInterests>>,
    channels: Arc<Mutex<channel::Manager>>,
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

    state.dispatcher.dispatch(&state.world);
    state.world.maintain();

    // // Send updates to interested client handlers.
    // // TODO: Do this in an ECS system.
    // let channels = channels.lock().unwrap();
    // for (addr, region) in client_interests.lock().unwrap().iter() {
    //     // TODO: Use region to determine what updates to send.
    //     let msg = channel::ClientHandlerMsg {
    //         cell_updates: vec![],
    //     };
    //     channels.send_to_client_handler(&addr, msg);
    // }

    Ok(())
}

// fn mock_cell_updates(counter: i32, region: &BoundingBox) -> Vec<channel::CellUpdate> {
//     let x_min = std::cmp::max(region.x_min as i32, 0);
//     let x_max = region.x_max as i32;
//     let y_min = std::cmp::max(region.y_min as i32, 0);
//     let y_max = region.y_max as i32;

//     let mut updates = vec![];
//     for x in x_min..x_max {
//         for y in y_min..y_max {
//             let grass = (((x + y) / 10) + (counter / 250)) % 5;
//             updates.push(channel::CellUpdate { x, y, grass });
//         }
//     }
//     updates
// }
