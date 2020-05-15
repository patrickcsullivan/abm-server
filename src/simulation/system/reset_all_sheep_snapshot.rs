use crate::simulation::grid::{CellBlock, CellBlockBuilder};
use crate::simulation::snapshot::AllSheepSnapshot;
use specs::prelude::*;

pub struct ResetAllSheepSnapshotSystem;

impl<'a> System<'a> for ResetAllSheepSnapshotSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = WriteExpect<'a, CellBlock<AllSheepSnapshot>>;

    fn run(&mut self, data: Self::SystemData) {
        let mut snapshot = data;
        // TODO: Implement and use a mutable CellBolock iterator. Don't take width and height params.
        *snapshot = CellBlockBuilder::new(16, 16, AllSheepSnapshot::default()).finish()
    }
}
