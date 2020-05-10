use crate::simulation::component::{Heading, Position, SheepBehaviorState, Velocity};

#[derive(Clone, Debug)]
pub struct CreateSheepCommand {
    pub position: Position,
    pub heading: Heading,
    pub velocity: Velocity,
    pub behavior: SheepBehaviorState,
}

#[derive(Debug)]
pub struct CreateSheepCommandQueue {
    pub commands: Vec<CreateSheepCommand>,
}

impl CreateSheepCommandQueue {
    pub fn new() -> CreateSheepCommandQueue {
        CreateSheepCommandQueue { commands: vec![] }
    }

    pub fn push(&mut self, command: CreateSheepCommand) {
        self.commands.push(command);
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}
