use spade::delaunay::DelaunayTreeLocate;
use spade::delaunay::FloatDelaunayTriangulation;
use spade::kernels::FloatKernel;
use spade::rtree::{NearestNeighborIterator, RTree};
use spade::{HasPosition, SpatialObject};
use specs::prelude::*;
use specs::Entity;

pub struct EntityPosition {
    pub entity: Entity,
    pub position: [f32; 2],
}

impl HasPosition for EntityPosition {
    type Point = [f32; 2];

    fn position(&self) -> [f32; 2] {
        self.position
    }
}

pub type EntityRTree = RTree<EntityPosition>;

// fn main(world: &mut World) -> EntityRTree {
//     let mut rtree = RTree::new();
//     let e = world.create_entity().build();
//     let epos = EntityPosition {
//         entity: e,
//         position: [0.0, 0.0],
//     };
//     rtree.insert(epos);
//     let mut iter = rtree.nearest_neighbor_iterator(&[0.0, 0.0]);
//     let mut lines: Vec<(f32, f32, bool)> = vec![];
//     for n in iter
//     rtree
// }

struct Line {
    x_intercept: f32,
    y_intercept: f32,
}

impl Line {
    fn is_vertical(&self) -> bool {
        !self.y_intercept.is_finite()
    }

    fn is_horizontal(&self) -> bool {
        !self.x_intercept.is_finite()
    }

    fn from_intercepts(x_intercept: f32, y_intercept: f32) -> Line {
        Line {
            x_intercept,
            y_intercept,
        }
    }

    fn from_point_slope(p: [f32; 2], slope: f32) -> Line {
        let [x, y] = p;
        if slope.abs() < std::f32::EPSILON {
            // horizontal line
            Line::from_intercepts(std::f32::NAN, y)
        } else if !slope.is_finite() {
            // vertical line
            Line::from_intercepts(x, std::f32::NAN)
        } else {
            let y_intercept = y - x * slope;
            let x_intercept = x - y / slope;
            Line::from_intercepts(x_intercept, y_intercept)
        }
    }

    fn from_points(p1: [f32; 2], p2: [f32; 2]) -> Line {
        let [x1, y1] = p1;
        let [x2, y2] = p2;
        if y2 - y1 < std::f32::EPSILON {
            // horizontal line
            Line::from_intercepts(std::f32::NAN, y1)
        } else if x2 - x1 < std::f32::EPSILON {
            // vertical line
            Line::from_intercepts(x1, std::f32::NAN)
        } else {
            let slope = (y2 - y1) / (x2 - x1);
            Line::from_point_slope(p1, slope)
        }
    }

    fn bisector(p1: [f32; 2], p2: [f32; 2]) -> Line {
        let [x1, y1] = p1;
        let [x2, y2] = p2;
        let mid = [x1 + (x2 - x1) / 2.0, y1 + (y2 - y1) / 2.0];
        let orthogonal_slope = -1.0 * (x2 - x1) / (y2 - y1);
        Line::from_point_slope(mid, orthogonal_slope)
    }
}
enum GreaterOrLess {
    Greater,
    Less,
}

struct LinearInequality {
    line: Line,
    comparator: GreaterOrLess,
}

impl LinearInequality {
    fn contains(&self, position: [f32; 2]) -> bool {
        let [x, y] = position;
        if !self.line.x_intercept.is_finite() {
            // horizontal line
            match self.comparator {
                GreaterOrLess::Greater => y > self.line.y_intercept,
                GreaterOrLess::Less => y < self.line.y_intercept,
            }
        } else if !self.line.y_intercept.is_finite() {
            // vertical line
            match self.comparator {
                GreaterOrLess::Greater => x > self.line.x_intercept,
                GreaterOrLess::Less => x < self.line.x_intercept,
            }
        } else {
            // Assume x and y intercepts are both finite.
            let slope = -1.0 * self.line.y_intercept / self.line.x_intercept;
            match self.comparator {
                GreaterOrLess::Greater => y > slope * x + self.line.y_intercept,
                GreaterOrLess::Less => y < slope * x + self.line.y_intercept,
            }
        }
    }
}

pub struct NaturalNeighborIterator<'a, T>
where
    T: SpatialObject + 'a,
{
    nearest: NearestNeighborIterator<'a, T>,

    /// Set of linear inequalities that describe the Vernoii polygon in which
    /// the query point exists.
    polygon_perimeter: Vec<LinearInequality>,
}

impl<'a> Iterator for NaturalNeighborIterator<'a, EntityPosition> {
    type Item = &'a EntityPosition;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(neighbor) = self.nearest.next() {
            if self
                .polygon_perimeter
                .iter()
                .all(|lin_ineq| lin_ineq.contains(neighbor.position))
            {
                let lin_ineq = LinearInequality {
                    line: Line::bisector(self.position, neighbor.position);
                    comparator: GreaterOrLess::Greater,
                };
                self.polygon_perimeter.push(lin_ineq);
            }
        }
        None
    }
}
