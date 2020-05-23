use crate::simulation::component::{Heading, Position, SheepBehaviorState};
use crate::simulation::entity_rtree::{EntityPosition, EntityRTree};
use spade::rtree::RTree;
use specs::prelude::*;

pub struct SheepRTreeSystem;

impl<'a> System<'a> for SheepRTreeSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, EntityRTree>,
        Entities<'a>,
        ReadStorage<'a, SheepBehaviorState>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Heading>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut rtree, entities, behavior_storage, pos_storage, heading_storage) = data;
        let entries = (&entities, &behavior_storage, &pos_storage, &heading_storage)
            .join()
            .map(|(entity, behavior, pos, heading)| {
                EntityPosition {
                    entity,
                    position: [pos.v.x, pos.v.y], // second nearest but not a natural neighbor
                    heading: *heading,
                    behavior: *behavior,
                }
            });
        *rtree = RTree::bulk_load(entries.collect());
    }
}
