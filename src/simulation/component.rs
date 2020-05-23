use nalgebra::{Rotation2, Vector2};
use specs::{prelude::*, Component};
use specs_derive::Component;
use std::net::SocketAddr;

#[derive(Clone, Copy, Component, Debug)]
pub struct Socket {
    pub addr: SocketAddr,
}

impl Socket {
    pub fn new(addr: SocketAddr) -> Socket {
        Socket { addr }
    }
}

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
#[derive(Clone, Copy, Component, Debug, PartialEq)]
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
#[derive(Clone, Copy, Component, Debug, PartialEq)]
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
    /// Indicates that the sheep is stationary and specifies the number of
    /// frames that have elapsed since it transitioned from running to
    /// stationary if the sheep's last behavior was running.
    Stationary {
        frames_since_running_to_stationary: Option<u64>,
    },
    Walking,
    Running,
}

/// Sheep behavior state.
#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub struct SheepBehaviorState {
    pub behavior: SheepBehavior,
    // pub next_check_millis: u16,
}

impl SheepBehaviorState {
    // pub const CHECK_PERIOD_MILLIS: u16 = 1000;

    pub fn new(behavior: SheepBehavior) -> SheepBehaviorState {
        SheepBehaviorState {
            behavior,
            // next_check_millis: SheepBehaviorState::CHECK_PERIOD_MILLIS,
        }
    }
}
