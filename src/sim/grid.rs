use std::convert::TryInto;

pub struct Grid<T> {
    width: u8,
    height: u8,
    cells: Vec<T>,
}

impl<T: Clone + Copy> Grid<T> {
    pub fn new(w: u8, h: u8, default: T) -> Grid<T> {
        Grid {
            width: w,
            height: h,
            cells: vec![default; w as usize * h as usize],
        }
    }

    pub fn set(&mut self, x: u8, y: u8, val: T) -> () {
        let idx = self.xy_idx(x, y);
        self.cells[idx] = val;
    }

    pub fn at(&self, x: u8, y: u8) -> Option<&T> {
        self.cells.get(self.xy_idx(x, y))
    }

    pub fn around(&self, x: u8, y: u8, range: u8) -> Vec<(u8, u8, &T)> {
        self.around_positions(x, y, range)
            .iter()
            .map(|(ax, ay)| (*ax, *ay, self.unsafe_at(*ax, *ay)))
            .collect()
    }

    pub fn within(&self, x_min: u8, x_max: u8, y_min: u8, y_max: u8) -> Vec<(u8, u8, &T)> {
        self.within_positions(x_min, x_max, y_min, y_max)
            .iter()
            .map(|(ax, ay)| (*ax, *ay, self.unsafe_at(*ax, *ay)))
            .collect()
    }

    pub fn all(&self) -> Vec<(u8, u8, &T)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let (x, y) = self.idx_xy(i);
                (x, y, c)
            })
            .collect()
    }

    pub fn all_positions(&self) -> Vec<(u8, u8)> {
        self.within_positions(0, self.width, 0, self.height)
    }

    // TODO: Make private.
    pub fn unsafe_at(&self, x: u8, y: u8) -> &T {
        let idx = self.xy_idx(x, y);
        &self.cells[idx]
    }

    /// Gets a list of positions around the specified cell.
    fn around_positions(&self, center_x: u8, center_y: u8, range: u8) -> Vec<(u8, u8)> {
        let x_min = if range < center_x {
            center_x - range
        } else {
            0
        };
        let x_max = std::cmp::min(self.width, center_x + range);
        let y_min = if range < center_y {
            center_y - range
        } else {
            0
        };
        let y_max = std::cmp::min(self.height, center_y + range);
        let horiz = (x_min..x_max).map(|x| (x, center_y));
        let vert = (y_min..y_max)
            .filter(|y| y.to_owned() != center_y)
            .map(|y| (center_x, y));
        return horiz.chain(vert).collect();
    }

    /// Gets a list of positions within the given bounds.
    fn within_positions(&self, x_min: u8, x_max: u8, y_min: u8, y_max: u8) -> Vec<(u8, u8)> {
        let x_max = std::cmp::min(self.width, x_max);
        let y_max = std::cmp::min(self.height, y_max);
        return (y_min..y_max)
            .flat_map(|y| (x_min..x_max).map(move |x| (x, y)))
            .collect();
    }

    fn xy_idx(&self, x: u8, y: u8) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn idx_xy(&self, idx: usize) -> (u8, u8) {
        ((idx as u8) % self.width, (idx as u8) / self.width)
    }
}
