use super::entity_rtree::EntityPosition;
use super::{
    command_queue::{CreateSheepCommand, CreateSheepCommandQueue},
    component,
    frame::Frame,
    grid::CellBlockBuilder,
    network, system,
};
use spade::rtree::RTree;
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
        State::initialize_rtree(&mut world);

        // Set up dispatcher and systems.
        let mut dispatcher = DispatcherBuilder::new()
            .with(system::DebugLogSystem, "debug_log", &[])
            // Process messages from inbox.
            .with(system::CreateSocketSystem, "create_port", &["debug_log"])
            // Clear and rebuild R-tree.
            .with(system::SheepRTreeSystem, "sheep_rtree", &["create_port"])
            // Update components.
            .with(
                system::SheepBehaviorSystem,
                "sheep_behavior",
                &["sheep_rtree"],
            )
            .with(
                system::SheepHeadingSystem,
                "sheep_heading",
                &["sheep_behavior"],
            )
            .with(
                system::SheepVelocitySystem,
                "sheep_velocity",
                &["sheep_heading"],
            )
            .with(system::PositionSystem, "position", &["sheep_velocity"])
            // Send messages to outbox.
            .with(system::OutboxSystem, "outbox", &["position"])
            // Execute commands to create adnd delete entities.
            .with(system::CreateCommandSystem, "create_command", &["outbox"])
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
                    position: component::Position::new((x * 4) as f32, (y * 4) as f32),
                    heading: component::Heading::new(0.0),
                    velocity: component::Velocity::new(0.0, 0.0),
                    behavior: component::SheepBehaviorState::new(component::SheepBehavior::Walking),
                });
            }
        }
        create_cmds.push(CreateSheepCommand {
            position: component::Position::new(64.0, 64.0),
            heading: component::Heading::new(0.0),
            velocity: component::Velocity::new(0.0, 0.0),
            behavior: component::SheepBehaviorState::new(component::SheepBehavior::Walking),
        });
        world.insert(create_cmds);
    }

    fn initialize_rtree(world: &mut World) {
        world.insert(RTree::<EntityPosition>::new());
    }
}
