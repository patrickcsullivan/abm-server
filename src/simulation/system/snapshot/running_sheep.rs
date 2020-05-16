use crate::simulation::component::{Heading, Position, SheepBehavior, SheepBehaviorState};
use crate::simulation::grid::{CellBlock, CellBlockBuilder, Grid};
use crate::simulation::snapshot::RunningSheepSnapshot;
use nalgebra::Vector2;
use specs::prelude::*;

pub struct RunningSheepSnapshotSystem;

impl<'a> System<'a> for RunningSheepSnapshotSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, CellBlock<RunningSheepSnapshot>>,
        ReadStorage<'a, SheepBehaviorState>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Heading>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut snapshots, behavior_storage, pos_storage, heading_storage) = data;
        // TODO: Implement and use a mutable CellBolock iterator. Don't take width and height params.
        *snapshots = CellBlockBuilder::new(16, 16, RunningSheepSnapshot::default()).finish();

        for (behavior, pos, heading) in (&behavior_storage, &pos_storage, &heading_storage).join() {
            if behavior.behavior == SheepBehavior::Running {
                let grid_pos = (pos.v.x as usize % 5, pos.v.y as usize % 5);
                let new_cell = snapshots.at(grid_pos).map(|c| {
                    let heading_vec = heading.r * Vector2::x();
                    RunningSheepSnapshot {
                        count: c.count + 1,
                        heading_sum: c.heading_sum + heading_vec,
                    }
                });
                if let Some(new_cell) = new_cell {
                    snapshots.set(grid_pos, new_cell);
                }
            }
        }
    }
}
