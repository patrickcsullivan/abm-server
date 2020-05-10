use crate::simulation::component::Position;
use crate::simulation::frame::DeltaFrame;
use specs::prelude::*;

pub struct DebugLogSystem;

impl<'a> System<'a> for DebugLogSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (ReadExpect<'a, DeltaFrame>, ReadStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (delta_resource, position_storage) = data;

        println!("Delta: {}", delta_resource.delta);

        let mut count = 0;
        for pos in (&position_storage).join() {
            count += 1;
            if count < 5 {
                println!("{{x:{}, y:{}}} ", pos.v.x, pos.v.y);
            }
        }
        println!("Entity Count: {}", count);
    }
}
