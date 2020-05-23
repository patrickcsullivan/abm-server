use std::time::{Duration, Instant};

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
    /// Duration of a fixed length frame in milliseconds.
    pub const DURATION_MILLIS: u64 = 16u64; // 16 ms = 62.5 FPS

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
            (now - self.ideal_start_time).as_millis() as u64 / Frame::DURATION_MILLIS;
        Frame {
            number: self.number + elapsed_frame_count,
            start_time: now,
            ideal_start_time: self.ideal_start_time
                + Duration::from_millis(elapsed_frame_count * Frame::DURATION_MILLIS),
        }
    }
}
