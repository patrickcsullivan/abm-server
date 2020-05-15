use nalgebra::Vector2;

/// Snapshot of information about all sheep in a cell.
#[derive(Clone, Copy, Debug)]
pub struct AllSheepSnapshot {
    /// The sum of heading vectors for all sheep in the cell.
    pub heading_sum: Vector2<f32>,
}

impl Default for AllSheepSnapshot {
    fn default() -> AllSheepSnapshot {
        AllSheepSnapshot {
            heading_sum: nalgebra::zero(),
        }
    }
}

// pub struct AllSheepSnapshot {
//     pub grid: CellBlock<AllSheepSnapshotCell>,
// }

// impl AllSheepSnapshot {
//     pub fn new(width: usize, height: usize) -> AllSheepSnapshot {
//         AllSheepSnapshot {
//             grid: CellBlockBuilder::new(width, height, AllSheepSnapshotCell::default()).finish(),
//         }
//     }
// }

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
