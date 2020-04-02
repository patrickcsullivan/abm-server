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
            cells: vec![default; (w * h) as usize],
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

    fn unsafe_at(&self, x: u8, y: u8) -> &T {
        let idx = self.xy_idx(x, y);
        &self.cells[idx]
    }

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

    fn xy_idx(&self, x: u8, y: u8) -> usize {
        ((y * self.width) + x).try_into().unwrap()
    }

    fn idx_xy(&self, idx: usize) -> (u8, u8) {
        ((idx as u8) % self.width, (idx as u8) / self.width)
    }
}
