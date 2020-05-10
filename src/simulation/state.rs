use super::{
    command_queue::{CreateSheepCommand, CreateSheepCommandQueue},
    component,
    frame::Frame,
    network, snapshot, system,
};
use specs::prelude::*;

pub struct State<'a, 'b> {
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'b>,
    pub frame: Option<Frame>,
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

        // Initialize resources.
        State::initialize_mailboxes(&mut world);
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

    fn initialize_mailboxes(world: &mut World) {
        let inbox: Vec<network::IncomingMessage> = vec![];
        world.insert(inbox);

        let outbox: Vec<network::OutgoingMessage> = vec![];
        world.insert(outbox);
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
