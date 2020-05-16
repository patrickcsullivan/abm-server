use crate::simulation::component::{Heading, Position, SheepBehavior, SheepBehaviorState};
use crate::simulation::grid::{CellBlock, Grid};
use crate::simulation::snapshot::AnySheepSnapshot;
use nalgebra::{Rotation2, Vector2};
use rand::distributions::{Distribution, Uniform};
use specs::prelude::*;

pub struct SheepHeadingSystem;

impl<'a> System<'a> for SheepHeadingSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, CellBlock<AnySheepSnapshot>>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, SheepBehaviorState>,
        WriteStorage<'a, Heading>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (sheep_snapshots, pos_storage, behavior_storage, mut heading_storage) = data;

        for (pos, behavior, mut heading) in
            (&pos_storage, &behavior_storage, &mut heading_storage).join()
        {
            match behavior.behavior {
                SheepBehavior::Stationary => {}
                SheepBehavior::Walking => {
                    heading.r = new_walking_heading(heading.r, pos.v, &sheep_snapshots);
                }
                SheepBehavior::Running => {}
            }
        }
    }
}

fn new_walking_heading(
    curr_heading: Rotation2<f32>,
    pos: Vector2<f32>,
    sheep_snapshots: &CellBlock<AnySheepSnapshot>,
) -> Rotation2<f32> {
    // TODO: Clean up.
    let grid_pos = (pos.x as usize % 5, pos.y as usize % 5);
    let cell = sheep_snapshots.at(grid_pos);

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
