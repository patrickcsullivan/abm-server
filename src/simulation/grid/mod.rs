mod cell_block;

pub use cell_block::{CellBlock, CellBlockBuilder};

/// An interface for dealing with grids of square cells.
pub trait Grid {
    type Cell;

    /// Gets the value of the cell at the given position.
    fn at(&self, pos: (usize, usize)) -> Option<&Self::Cell>;

    /// Sets the value of the cell at the given positin.
    fn set(&mut self, pos: (usize, usize), t: Self::Cell);

    /// Creates an iterator which yields "visible" cells within a specified
    /// Manhattan distance that satisfy the given predicate. Cells are ordered by
    /// ascending Manhattan distance and then clockwise order starting from
    /// the bottom left cell.
    fn visible_neighbors<P>(
        &self,
        pos: (usize, usize),
        max_dist: usize,
        predicate: P,
    ) -> VisibleNeighborSearch<Self, P>
    where
        Self: Sized,
        P: FnMut((usize, usize), &Self::Cell) -> bool,
    {
        VisibleNeighborSearch::new(self, pos, max_dist, predicate)
    }

    /// Creates an iterator which yields cells within a specified Manhattan
    /// distance that satisfy the given predicate. Cells are ordered by
    /// ascending Manhattan distance and then clockwise order starting from
    /// the bottom left cell.
    fn neighbors<P>(
        &self,
        pos: (usize, usize),
        max_dist: usize,
        predicate: P,
    ) -> NeighborSearch<Self, P>
    where
        Self: Sized,
        P: FnMut((usize, usize), &Self::Cell) -> bool,
    {
        NeighborSearch::new(self, pos, max_dist, predicate)
    }
}

pub struct VisibleNeighborSearch<'a, G, P> {
    /// Grid of square cells to search.
    grid: &'a G,

    /// The cell around which to search.
    center: (usize, usize),

    /// Maximum Manhattan distance from the center cell.
    max_dist: usize,

    /// Critera that a cell must meet to be a match in the search.
    predicate: P,

    /// Current Manhattan distance from the center cell.
    curr_dist: usize,

    /// List of indices into the current search shell indicating which cells are
    /// visible and can be searched.
    visible: Vec<usize>,

    /// Index into `visible` that indicates which cell should be searched next.
    next_check: usize,

    /// List of indices into the current shell that have mathced the search
    /// critera. These matches will block the visibility of cells in outer
    /// shells.
    matches: Vec<usize>,
}

impl<G, P> VisibleNeighborSearch<'_, G, P> {
    /// Creates a new search for visible neighbors.
    pub fn new(
        grid: &G,
        center: (usize, usize),
        max_dist: usize,
        predicate: P,
    ) -> VisibleNeighborSearch<G, P> {
        VisibleNeighborSearch {
            grid,
            center,
            max_dist,
            predicate,
            curr_dist: 0,
            visible: vec![0],
            next_check: 0,
            matches: vec![],
        }
    }
}

impl<'a, G: Grid, P> Iterator for VisibleNeighborSearch<'a, G, P>
where
    P: FnMut((usize, usize), &G::Cell) -> bool,
{
    type Item = ((usize, usize), &'a G::Cell);

    fn next(&mut self) -> Option<((usize, usize), &'a G::Cell)> {
        if self.next_check < self.visible.len() {
            let idx = self.visible[self.next_check];
            self.next_check += 1;
            if let Some(pos) = search_ring::idx_pos(idx, self.center, self.curr_dist) {
                if let Some(cell) = self.grid.at(pos) {
                    if (self.predicate)(pos, cell) {
                        // Mark cell as matched so it blocks cells in outer shells.
                        self.matches.push(idx);
                        return Some((pos, cell.to_owned()));
                    }
                }
            }
            self.next()
        } else if self.curr_dist < self.max_dist {
            self.visible = search_ring::next_visible(self.curr_dist, &self.visible, &self.matches);
            self.matches = vec![];
            self.curr_dist += 1;
            self.next_check = 0;
            self.next()
        } else {
            None
        }
    }
}

pub struct NeighborSearch<'a, G, P> {
    /// Grid of square cells to search.
    grid: &'a G,

    /// The cell around which to search.
    center: (usize, usize),

    /// Maximum Manhattan distance from the center cell.
    max_dist: usize,

    /// Critera that a cell must meet to be a match in the search.
    predicate: P,

    /// Current Manhattan distance from the center cell.
    curr_dist: usize,

    /// Index into the current search ring.
    curr_ring_idx: usize,
}

impl<G, P> NeighborSearch<'_, G, P> {
    /// Creates a new search for visible neighbors.
    pub fn new(
        grid: &G,
        center: (usize, usize),
        max_dist: usize,
        predicate: P,
    ) -> NeighborSearch<G, P> {
        NeighborSearch {
            grid,
            center,
            max_dist,
            predicate,
            curr_dist: 0,
            curr_ring_idx: 0,
        }
    }
}

impl<'a, G: Grid, P> Iterator for NeighborSearch<'a, G, P>
where
    P: FnMut((usize, usize), &G::Cell) -> bool,
{
    type Item = ((usize, usize), &'a G::Cell);

    fn next(&mut self) -> Option<((usize, usize), &'a G::Cell)> {
        if (self.curr_dist == 0 && self.curr_ring_idx < 1)
            || self.curr_ring_idx < 8 * self.curr_dist
        {
            let curr_idx = self.curr_ring_idx;
            self.curr_ring_idx += 1;
            if let Some(pos) = search_ring::idx_pos(curr_idx, self.center, self.curr_dist) {
                if let Some(cell) = self.grid.at(pos) {
                    if (self.predicate)(pos, cell) {
                        return Some((pos, cell.to_owned()));
                    }
                }
            }
            self.next()
        } else if self.curr_dist < self.max_dist {
            self.curr_dist += 1;
            self.curr_ring_idx = 0;
            self.next()
        } else {
            None
        }
    }
}

mod search_ring {
    /// Gets list of indices into the next ring of cells indicating which cells
    /// are visible and can be searched.
    pub fn next_visible(
        curr_dist: usize,
        curr_visible: &[usize],
        curr_matches: &[usize],
    ) -> Vec<usize> {
        if curr_dist == 0 {
            if curr_matches.is_empty() {
                return (0..8).collect();
            } else {
                return vec![];
            }
        }

        // If a cell from the current ring was a match or adjacent to a matched
        // cell then it blocks visibility into the next ring.
        let blocked: Vec<usize> = curr_matches
            .iter()
            .flat_map(|&i| {
                let shell_len = 8 * curr_dist;
                if i == 0 {
                    vec![shell_len - 1, i, (i + 1) % shell_len]
                } else {
                    vec![i - 1, i, (i + 1) % shell_len]
                }
            })
            .collect();

        // If a cell from the current ring does not block visibility then
        // include cells "behind" it in the next ring of visible cells.
        curr_visible
            .iter()
            .filter(|i| !blocked.contains(i))
            .flat_map(|i| {
                let j = i + i / curr_dist;
                if i % curr_dist == 0 {
                    vec![j, j + 1]
                } else {
                    vec![j + 1]
                }
            })
            .collect()
    }

    /// Converts the cell ring index into a position on a grid.
    pub fn idx_pos(idx: usize, center: (usize, usize), dist: usize) -> Option<(usize, usize)> {
        if dist == 0 {
            return Some(center);
        }

        let (x, y) = center;
        let rel_idx = idx % (2 * dist);

        if idx < 2 * dist {
            // bottom side
            if y < dist || x + rel_idx < dist {
                None
            } else {
                Some((x + rel_idx - dist, y - dist))
            }
        } else if idx < 4 * dist {
            // right side
            if y + rel_idx < dist {
                None
            } else {
                Some((x + dist, y + rel_idx - dist))
            }
        } else if idx < 6 * dist {
            // top side
            if x + dist < rel_idx {
                None
            } else {
                Some((x + dist - rel_idx, y + dist))
            }
        } else if idx < 8 * dist {
            // left side
            if x < dist || y + dist < rel_idx {
                None
            } else {
                Some((x - dist, y + dist - rel_idx))
            }
        } else {
            None
        }
    }

    #[cfg(test)]
    mod tests {
        use super::idx_pos;

        #[test]
        fn idx_pos_bottom_left() {
            assert_eq!(idx_pos(0, (3, 3), 3), Some((0, 0)));
        }

        #[test]
        fn idx_pos_bottom() {
            assert_eq!(idx_pos(1, (3, 3), 3), Some((1, 0)));
        }

        #[test]
        fn idx_pos_bottom_right() {
            assert_eq!(idx_pos(6, (3, 3), 3), Some((6, 0)));
        }

        #[test]
        fn idx_pos_right() {
            assert_eq!(idx_pos(7, (3, 3), 3), Some((6, 1)));
        }

        #[test]
        fn idx_pos_top_right() {
            assert_eq!(idx_pos(12, (3, 3), 3), Some((6, 6)));
        }

        #[test]
        fn idx_pos_top() {
            assert_eq!(idx_pos(13, (3, 3), 3), Some((5, 6)));
        }

        #[test]
        fn idx_pos_top_left() {
            assert_eq!(idx_pos(18, (3, 3), 3), Some((0, 6)));
        }

        #[test]
        fn idx_pos_left() {
            assert_eq!(idx_pos(19, (3, 3), 3), Some((0, 5)));
        }

        #[test]
        fn idx_pos_negative_x() {
            // The bottom left corner would be at (-1, 0) but position
            // components are unsigned.
            assert_eq!(idx_pos(0, (2, 3), 3), None);

            assert_eq!(idx_pos(1, (2, 3), 3), Some((0, 0)));
        }

        #[test]
        fn idx_pos_negative_y() {
            // The bottom left corner would be at (0, -1) but position
            // components are unsigned.
            assert_eq!(idx_pos(0, (3, 2), 3), None);

            assert_eq!(idx_pos(23, (3, 2), 3), Some((0, 0)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CellBlock, CellBlockBuilder, Grid};

    #[test]
    fn visible_neighbors_with_center_match() {
        let mut grid: CellBlock<bool> = CellBlockBuilder::new(7, 7, false).finish();
        grid.set((3, 3), true);
        grid.set((2, 2), true);
        let mut search = grid.visible_neighbors((3, 3), 2, |_, &t| t);

        // Expect match at center.
        assert_eq!(search.next(), Some(((3, 3), &true)));

        // Expect potential match outside center to not be found. It is blocked
        // by match at center.
        assert_eq!(search.next(), None);
    }

    #[test]
    fn visible_neighbors_blocked() {
        // cells that match predicate
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 1 0 X 0 0 0
        // 0 0 1 0 0 0 0
        // 0 0 0 0 1 0 0
        // 0 0 0 0 0 0 0
        let mut grid: CellBlock<bool> = CellBlockBuilder::new(7, 7, false).finish();
        grid.set((1, 3), true);
        grid.set((2, 2), true);
        grid.set((4, 1), true);
        let mut search = grid.visible_neighbors((3, 3), 2, |_, &t| t);

        // Expect potential match at (2, 2) to be found. It is visible.
        assert_eq!(search.next(), Some(((2, 2), &true)));

        // Expect other potential matches to not be found. They are blocked by
        // match at (2, 2).
        assert_eq!(search.next(), None);
    }

    #[test]
    fn visible_neighbors_unblocked() {
        // cells that match predicate
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 1 0 0 0 0 0
        // 0 0 0 X 0 0 0
        // 0 0 1 0 0 0 0
        // 0 0 0 0 0 1 0
        // 0 0 0 0 0 0 0
        let mut grid: CellBlock<bool> = CellBlockBuilder::new(7, 7, false).finish();
        grid.set((1, 4), true);
        grid.set((2, 2), true);
        grid.set((5, 1), true);
        let mut search = grid.visible_neighbors((3, 3), 2, |_, &t| t);

        // Expect potential match at (2, 2) to be found. It is visible.
        assert_eq!(search.next(), Some(((2, 2), &true)));

        // Expect other potential matches to be found in clockwise order
        // starting from bottom left. They are not blocked by match at (2, 2).
        assert_eq!(search.next(), Some(((5, 1), &true)));
        assert_eq!(search.next(), Some(((1, 4), &true)));
    }

    #[test]
    fn visible_neighbors_range() {
        // cells that match predicate
        // 0 0 0 0 0 0 1
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 X 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 0 0
        let mut grid: CellBlock<bool> = CellBlockBuilder::new(7, 7, false).finish();
        grid.set((2, 2), true);
        grid.set((6, 6), true);
        let mut search = grid.visible_neighbors((3, 3), 2, |_, &t| t);

        // Expect potential match at (2, 2) to be found. It is visible.
        assert_eq!(search.next(), Some(((2, 2), &true)));

        // Expect other potential match to not be found. It is too far.
        assert_eq!(search.next(), None);
    }

    #[test]
    fn neighbors() {
        // cells that match predicate
        // 0 0 0 1 0 0 0
        // 0 0 0 0 0 0 0
        // 0 0 0 0 0 1 0
        // 0 0 0 X 0 0 0
        // 0 0 1 0 0 0 0
        // 0 0 0 1 0 0 0
        // 0 0 0 0 0 0 0
        let mut grid: CellBlock<bool> = CellBlockBuilder::new(7, 7, false).finish();
        grid.set((2, 2), true);
        grid.set((3, 1), true);
        grid.set((5, 4), true);
        grid.set((3, 7), true);
        let mut search = grid.neighbors((3, 3), 2, |_, &t| t);

        assert_eq!(search.next(), Some(((2, 2), &true)));
        assert_eq!(search.next(), Some(((3, 1), &true)));
        assert_eq!(search.next(), Some(((5, 4), &true)));

        // Expect the potential match at (3, 7) to not be found. It is too far.
        assert_eq!(search.next(), None);
    }
}
