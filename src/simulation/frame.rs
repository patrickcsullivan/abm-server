use std::time::{Duration, Instant};

/// Ratio of time elapsed in reality to elapsed time modelled in the
/// simulation. For example if this value is set to 2.0 and one second
/// passes in reality then 2.0 seconds should elapse in the simulation.  
pub const REAL_TO_SIM_TIME: f32 = 1.0;

/// Duration of a fixed length frame in milliseconds.
pub const FRAME_DURATION_MILLIS: u64 = 16u64; // 16 ms = 62.5 FPS

/// The number of frames that have passed since the last simulation.
#[derive(Clone, Copy, Debug)]
pub struct DeltaFrame {
    pub delta: u64,
}

impl DeltaFrame {
    pub fn new(delta: u64) -> DeltaFrame {
        DeltaFrame { delta }
    }
}

// A counter that tracks the number and timing of a fixed length frame.
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    /// The number of the command frame.
    pub number: u64,

    /// The time at which the frame started.
    pub start_time: Instant,

    /// The ideal time at which the frame should have started. This is used to
    /// make sure sure that the start time of future frames don't cummulatively
    /// drift from the ideal start times.
    pub ideal_start_time: Instant,
}

impl Frame {
    pub fn new() -> Frame {
        let now = Instant::now();
        Frame {
            number: 0,
            start_time: now,
            ideal_start_time: now,
        }
    }

    pub fn next(&self, now: Instant) -> Frame {
        let elapsed_frame_count =
            (now - self.ideal_start_time).as_millis() as u64 / FRAME_DURATION_MILLIS;
        Frame {
            number: self.number + elapsed_frame_count,
            start_time: now,
            ideal_start_time: self.ideal_start_time
                + Duration::from_millis(elapsed_frame_count * FRAME_DURATION_MILLIS),
        }
    }
}
