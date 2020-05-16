use crate::simulation::{
    component::{Position, SheepBehavior, SheepBehaviorState},
    grid::{CellBlock, CellBlockBuilder, Grid},
    snapshot::RunningToStationarySheepSnapshot,
};
use specs::prelude::*;

pub struct RunningToStationarySheepSnapshotSystem;

impl<'a> System<'a> for RunningToStationarySheepSnapshotSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, CellBlock<RunningToStationarySheepSnapshot>>,
        ReadStorage<'a, SheepBehaviorState>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut snapshots, behavior_storage, pos_storage) = data;
        // TODO: Implement and use a mutable CellBolock iterator. Don't take width and height params.
        *snapshots =
            CellBlockBuilder::new(16, 16, RunningToStationarySheepSnapshot::default()).finish();

        for (behavior, pos) in (&behavior_storage, &pos_storage).join() {
            if let SheepBehavior::Stationary {
                was_running_last_update: true,
            } = behavior.behavior
            {
                let grid_pos = (pos.v.x as usize % 5, pos.v.y as usize % 5);
                let new_cell = snapshots
                    .at(grid_pos)
                    .map(|c| RunningToStationarySheepSnapshot { count: c.count + 1 });
                if let Some(new_cell) = new_cell {
                    snapshots.set(grid_pos, new_cell);
                }
            }
        }
    }
}
