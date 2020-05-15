use crate::simulation::component::{Heading, Position, SheepBehaviorState};
use crate::simulation::grid::{CellBlock, Grid};
use crate::simulation::snapshot::AllSheepSnapshot;
use nalgebra::Vector2;
use specs::prelude::*;

pub struct AllSheepSnapshotSystem;

impl<'a> System<'a> for AllSheepSnapshotSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, CellBlock<AllSheepSnapshot>>,
        ReadStorage<'a, SheepBehaviorState>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Heading>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut sheep_snapshots, behavior_storate, pos_storage, heading_storage) = data;

        for (_, pos, heading) in (&behavior_storate, &pos_storage, &heading_storage).join() {
            let grid_pos = (pos.v.x as usize % 5, pos.v.y as usize % 5);
            let new_cell = sheep_snapshots.at(grid_pos).map(|c| {
                let heading_vec = heading.r * Vector2::x();
                AllSheepSnapshot {
                    heading_sum: c.heading_sum + heading_vec,
                }
            });
            if let Some(new_cell) = new_cell {
                sheep_snapshots.set(grid_pos, new_cell);
            }
        }
    }
}
