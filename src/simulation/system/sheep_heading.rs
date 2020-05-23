use crate::simulation::{
    component::{Heading, Position, SheepBehavior, SheepBehaviorState},
    entity_rtree::{EntityPosition, EntityRTree, IntoNaturalNeighborIterator},
};
use nalgebra::{Rotation2, Vector2};
use rand::distributions::{Distribution, Uniform};
use specs::prelude::*;

pub struct SheepHeadingSystem;

impl<'a> System<'a> for SheepHeadingSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, EntityRTree>,
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, SheepBehaviorState>,
        WriteStorage<'a, Heading>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (rtree, entities, pos_storage, behavior_storage, mut heading_storage) = data;

        for (entity, pos, behavior, mut heading) in (
            &entities,
            &pos_storage,
            &behavior_storage,
            &mut heading_storage,
        )
            .join()
        {
            match behavior.behavior {
                SheepBehavior::Stationary { .. } => {}
                SheepBehavior::Walking => {
                    heading.r = new_walking_heading(heading.r, pos.v, &rtree);
                }
                SheepBehavior::Running => {
                    heading.r = new_running_heading(entity, pos.v, &rtree);
                }
            }
        }
    }
}

fn new_walking_heading(
    curr_heading: Rotation2<f32>,
    pos: Vector2<f32>,
    rtree: &EntityRTree,
) -> Rotation2<f32> {
    /// The distance in meters within which non-running sheep will influence each
    /// other's behavior. Represented as r0 in the literature.
    const METRIC_INTERACTION_RANGE: f32 = 1.0;

    let metric_neighbors: Vec<&EntityPosition> =
        rtree.lookup_in_circle(&[pos.x, pos.y], &METRIC_INTERACTION_RANGE);

    let heading_vec_sum: Vector2<f32> = metric_neighbors
        .into_iter()
        .map(|epos| epos.heading.r * Vector2::x())
        .sum();

    let next_heading_without_noise = if heading_vec_sum.magnitude() > 0.1 {
        Rotation2::rotation_between(&Vector2::x(), &heading_vec_sum)
    } else {
        curr_heading
    };

    // Add some noise to get the new heading.
    let mut rng = rand::thread_rng();
    const NOISE: f32 = 0.4082; // PI * 0.13
    let noise_angle = Uniform::from(-1.0 * NOISE..NOISE).sample(&mut rng);
    let noise_rot: Rotation2<f32> = Rotation2::new(noise_angle);
    next_heading_without_noise * noise_rot
}

fn new_running_heading(entity: Entity, pos: Vector2<f32>, rtree: &EntityRTree) -> Rotation2<f32> {
    // A constant paramter that affects how strongly a running sheep will align
    // its heading with other running sheep.
    const ALIGNMENT_MIMETIC_EFFECT: f32 = 4.0;

    // Relative strength of the force that pushes or pulls running sheem to an
    // equilibrium distance.
    const EQB_FORCE_STENGTH: f32 = 0.8;

    let running_natural_neighbors: Vec<&EntityPosition> = rtree
        .natural_neighbor_iterator(&[pos.x, pos.y], |epos| {
            epos.entity != entity && epos.behavior.behavior == SheepBehavior::Running
        })
        .take(4)
        .collect();

    let next_heading_vec = running_natural_neighbors
        .into_iter()
        .map(|epos| {
            let heading_vec = epos.heading.r * Vector2::x();
            let heading_component = ALIGNMENT_MIMETIC_EFFECT * heading_vec;
            let pos_to_neighbor = Vector2::new(epos.position[0] - pos.x, epos.position[1] - pos.y);
            let dist = pos_to_neighbor.magnitude();
            let eqb_component =
                EQB_FORCE_STENGTH * equilibrium_force(dist) * pos_to_neighbor / dist;
            heading_component + eqb_component
        })
        .sum();

    Rotation2::rotation_between(&Vector2::x(), &next_heading_vec)
}

/// Force that pulls sheep together when they are further than the equilibrium
/// distance and pushes them apart when they are closer than the equilibrium
/// distance.
fn equilibrium_force(dist: f32) -> f32 {
    const EQB_DIST: f32 = 1.0;
    let f = (dist - EQB_DIST) / EQB_DIST;
    if f > 1.0 {
        1.0
    } else {
        f
    }
}
