use crate::simulation::{
    component::{Heading, Position, SheepBehavior, SheepBehaviorState},
    grid::{CellBlock, Grid},
    snapshot::{AnySheepSnapshot, RunningSheepSnapshot},
};
use nalgebra::{Rotation2, Vector2};
use rand::{
    distributions::{Distribution, Uniform},
    prelude::*,
};
use specs::prelude::*;

pub struct SheepHeadingSystem;

impl<'a> System<'a> for SheepHeadingSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, CellBlock<AnySheepSnapshot>>,
        ReadExpect<'a, CellBlock<RunningSheepSnapshot>>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, SheepBehaviorState>,
        WriteStorage<'a, Heading>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            any_sheep_snapshots,
            running_snapshots,
            pos_storage,
            behavior_storage,
            mut heading_storage,
        ) = data;

        for (pos, behavior, mut heading) in
            (&pos_storage, &behavior_storage, &mut heading_storage).join()
        {
            match behavior.behavior {
                SheepBehavior::Stationary { .. } => {}
                SheepBehavior::Walking => {
                    heading.r = new_walking_heading(heading.r, pos.v, &any_sheep_snapshots);
                }
                SheepBehavior::Running => {
                    heading.r = new_running_heading(pos.v, &running_snapshots);
                }
            }
        }
    }
}

fn new_walking_heading(
    curr_heading: Rotation2<f32>,
    pos: Vector2<f32>,
    any_sheep_snapshots: &CellBlock<AnySheepSnapshot>,
) -> Rotation2<f32> {
    // TODO: Factor out.
    let grid_pos = (pos.x as usize % 5, pos.y as usize % 5);
    let cell = any_sheep_snapshots.at(grid_pos);

    // Get the mean heading from current cell.
    let next_without_noise = match cell {
        Some(AnySheepSnapshot {
            heading_sum: h_sum, ..
        }) if h_sum.magnitude() > 0.1 => Rotation2::rotation_between(&Vector2::x(), h_sum),
        _ => curr_heading,
    };

    // Add some noise to get the new heading.
    let mut rng = rand::thread_rng();
    const NOISE: f32 = 0.4082; // PI * 0.13
    let noise_angle = Uniform::from(-1.0 * NOISE..NOISE).sample(&mut rng);
    let noise_rot: Rotation2<f32> = Rotation2::new(noise_angle);
    next_without_noise * noise_rot
}

fn new_running_heading(
    pos: Vector2<f32>,
    running_snapshots: &CellBlock<RunningSheepSnapshot>,
) -> Rotation2<f32> {
    // TODO: Factor out.
    let grid_pos = (pos.x as usize % 5, pos.y as usize % 5);

    // A constant paramter that affects how strongly a running sheep will align
    // its heading with other running sheep.
    const ALIGNMENT_MIMETIC_EFFECT: f32 = 4.0;

    // Relative strength of the force that pushes or pulls running sheem to an
    // equilibrium distance.
    const EQB_FORCE_STENGTH: f32 = 0.8;

    let vn = running_snapshots.visible_neighbors(grid_pos, 8, |pos, cell| {
        pos == grid_pos && cell.count > 1 || pos != grid_pos && cell.count > 0
    });

    let mut rng = rand::thread_rng();

    let heading_vec: Vector2<f32> = vn.fold(nalgebra::zero(), |accum, ((x, y), snapshot)| {
        let alignmnet_component =
            ALIGNMENT_MIMETIC_EFFECT / snapshot.count as f32 * snapshot.heading_sum;
        let cell_center = Vector2::new(
            (x * 5) as f32 + rng.gen::<f32>() * 5.0,
            (y * 5) as f32 + rng.gen::<f32>() * 5.0,
        );
        // let cell_center = Vector2::new((x * 5) as f32 + 2.5, (y * 5) as f32 + 2.5);
        let pos_to_cell = cell_center - pos;
        let dist = (pos_to_cell.x * pos_to_cell.x + pos_to_cell.y * pos_to_cell.y).sqrt();
        let eqb_dist_component = EQB_FORCE_STENGTH * equilibrium_force(dist) * pos_to_cell;

        accum + alignmnet_component + eqb_dist_component
    });

    Rotation2::rotation_between(&Vector2::x(), &heading_vec)
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
