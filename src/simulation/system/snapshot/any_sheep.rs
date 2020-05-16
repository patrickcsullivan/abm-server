use crate::simulation::component::{Heading, Position, SheepBehaviorState};
use crate::simulation::grid::{CellBlock, CellBlockBuilder, Grid};
use crate::simulation::snapshot::AnySheepSnapshot;
use nalgebra::Vector2;
use specs::prelude::*;

pub struct AnySheepSnapshotSystem;

impl<'a> System<'a> for AnySheepSnapshotSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, CellBlock<AnySheepSnapshot>>,
        ReadStorage<'a, SheepBehaviorState>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Heading>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut snapshots, behavior_storage, pos_storage, heading_storage) = data;
        // TODO: Implement and use a mutable CellBolock iterator. Don't take width and height params.
        *snapshots = CellBlockBuilder::new(16, 16, AnySheepSnapshot::default()).finish();

        for (_, pos, heading) in (&behavior_storage, &pos_storage, &heading_storage).join() {
            let grid_pos = (pos.v.x as usize % 5, pos.v.y as usize % 5);
            let new_cell = snapshots.at(grid_pos).map(|c| {
                let heading_vec = heading.r * Vector2::x();
                AnySheepSnapshot {
                    heading_sum: c.heading_sum + heading_vec,
                }
            });
            if let Some(new_cell) = new_cell {
                snapshots.set(grid_pos, new_cell);
            }
        }
    }
}
