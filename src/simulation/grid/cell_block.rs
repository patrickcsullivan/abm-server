use super::Grid;

pub struct CellBlockBuilder<T> {
    width: usize,
    height: usize,
    default: T,
}

impl<T: Clone + Copy> CellBlockBuilder<T> {
    pub fn new(width: usize, height: usize, default: T) -> CellBlockBuilder<T> {
        CellBlockBuilder {
            width,
            height,
            default,
        }
    }

    pub fn finish(&self) -> CellBlock<T> {
        CellBlock {
            width: self.width,
            cells: vec![self.default; (self.width * self.height) as usize],
        }
    }
}

pub struct CellBlock<T> {
    width: usize,
    cells: Vec<T>,
}

impl<T> CellBlock<T> {
    pub fn pos_idx(&self, pos: (usize, usize)) -> usize {
        let (x, y) = pos;
        y * self.width + x
    }
}

impl<T> Grid for CellBlock<T> {
    type Cell = T;

    fn at(&self, pos: (usize, usize)) -> Option<&T> {
        let idx = self.pos_idx(pos);
        self.cells.get(idx)
    }

    fn set(&mut self, pos: (usize, usize), t: T) {
        let idx = self.pos_idx(pos);
        if idx < self.cells.len() {
            self.cells[idx] = t;
        }
    }
}
