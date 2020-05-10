use nalgebra::{Rotation2, Vector2};
use specs::{prelude::*, Component};
use specs_derive::Component;

/// Position in meters.
#[derive(Clone, Copy, Component, Debug)]
pub struct Position {
    pub v: Vector2<f32>,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position {
            v: Vector2::new(x, y),
        }
    }
}

/// Heading rotation.
#[derive(Clone, Copy, Component, Debug)]
pub struct Heading {
    pub r: Rotation2<f32>,
}

impl Heading {
    pub fn new(angle: f32) -> Heading {
        Heading {
            r: Rotation2::new(angle),
        }
    }
}

/// Velocity in meters per second.
#[derive(Clone, Copy, Component, Debug)]
pub struct Velocity {
    pub v: Vector2<f32>,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity {
            v: Vector2::new(x, y),
        }
    }
}

/// Types of sheep behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SheepBehavior {
    Stationary,
    Walking,
    Running,
}

/// Sheep behavior state.
#[derive(Clone, Copy, Component, Debug)]
pub struct SheepBehaviorState {
    pub behavior: SheepBehavior,
}

impl SheepBehaviorState {
    pub fn new(behavior: SheepBehavior) -> SheepBehaviorState {
        SheepBehaviorState { behavior }
    }
}
