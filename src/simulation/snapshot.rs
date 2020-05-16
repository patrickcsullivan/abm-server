use nalgebra::Vector2;

/// Snapshot of information about all sheep in a cell.
#[derive(Clone, Copy, Debug)]
pub struct AnySheepSnapshot {
    /// The sum of heading vectors for all sheep in the cell.
    pub heading_sum: Vector2<f32>,
}

impl Default for AnySheepSnapshot {
    fn default() -> AnySheepSnapshot {
        AnySheepSnapshot {
            heading_sum: nalgebra::zero(),
        }
    }
}

/// Snapshot of information about stationary sheep in a cell.
#[derive(Clone, Copy, Debug)]
pub struct StationarySheepSnapshot {
    /// The number of stationary sheep in the cell.
    pub count: u16,
}

impl Default for StationarySheepSnapshot {
    fn default() -> StationarySheepSnapshot {
        StationarySheepSnapshot { count: 0 }
    }
}

/// Snapshot of information about walking sheep in a cell.
#[derive(Clone, Copy, Debug)]
pub struct WalkingSheepSnapshot {
    /// The number of walking sheep in the cell.
    pub count: u16,
}

impl Default for WalkingSheepSnapshot {
    fn default() -> WalkingSheepSnapshot {
        WalkingSheepSnapshot { count: 0 }
    }
}

/// Snapshot of information about running sheep in a cell.
#[derive(Clone, Copy, Debug)]
pub struct RunningSheepSnapshot {
    /// The number of running sheep in the cell.
    pub count: u16,

    /// The sum of heading vectors for all running sheep in the cell.
    pub heading_sum: Vector2<f32>,
}

impl Default for RunningSheepSnapshot {
    fn default() -> RunningSheepSnapshot {
        RunningSheepSnapshot {
            count: 0,
            heading_sum: nalgebra::zero(),
        }
    }
}

/// Snapshot of information about sheep in the cell that transitioned from
/// running to stationary on their last behavior update.
#[derive(Clone, Copy, Debug)]
pub struct RunningToStationarySheepSnapshot {
    /// The number of sheep in the cell that transitioned from running to
    /// stationary on their last behavior update.
    pub count: u16,
}

impl Default for RunningToStationarySheepSnapshot {
    fn default() -> RunningToStationarySheepSnapshot {
        RunningToStationarySheepSnapshot { count: 0 }
    }
}
