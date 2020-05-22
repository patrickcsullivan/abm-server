use spade::rtree::{NearestNeighborIterator, RTree};
use spade::{HasPosition, SpatialObject};
use specs::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug)]
enum Line {
    NonVertical { slope: f32, y_intercept: f32 },
    Vertical { x_intercept: f32 },
}

impl Line {
    fn bisector(p1: [f32; 2], p2: [f32; 2]) -> Line {
        let [x1, y1] = p1;
        let [x2, y2] = p2;
        if x2 - x1 < std::f32::EPSILON {
            Line::NonVertical {
                slope: 0.0,
                y_intercept: y1 + (y2 - y1) / 2.0,
            }
        } else if y2 - y1 < std::f32::EPSILON {
            Line::Vertical {
                x_intercept: x1 + (x2 - x1) / 2.0,
            }
        } else {
            let orthogonal_slope = -1.0 * (x2 - x1) / (y2 - y1);
            let y_intercept = y1 - orthogonal_slope * x1;
            Line::NonVertical {
                slope: orthogonal_slope,
                y_intercept,
            }
        }
    }
}

fn is_less_than(p: [f32; 2], line: &Line) -> bool {
    let [x, y] = p;
    match line {
        Line::NonVertical { slope, y_intercept } => y < slope * x + y_intercept,
        Line::Vertical { x_intercept } => x < *x_intercept,
    }
}

pub trait IntoNaturalNeighborIterator<T>
where
    T: HasPosition,
{
    fn natural_neighbor_iterator(&self, query_point: &[f32; 2]) -> NaturalNeighborIterator<T>;
}

impl IntoNaturalNeighborIterator<EntityPosition> for EntityRTree {
    fn natural_neighbor_iterator(
        &self,
        query_point: &[f32; 2],
    ) -> NaturalNeighborIterator<EntityPosition> {
        NaturalNeighborIterator::new(self, *query_point)
    }
}

pub struct NaturalNeighborIterator<'a, T>
where
    T: SpatialObject + 'a,
{
    nearest: NearestNeighborIterator<'a, T>,

    query_point: T::Point,

    /// Set of lines that describe the Vernoii polygon in which the query point
    /// exists.
    polygon_perimeter: Vec<Line>,
}

impl<'a, T> NaturalNeighborIterator<'a, T>
where
    T: SpatialObject + 'a,
{
    fn new(rtree: &'a RTree<T>, query_point: T::Point) -> Self {
        let nearest = rtree.nearest_neighbor_iterator(&query_point);
        NaturalNeighborIterator {
            nearest,
            query_point,
            polygon_perimeter: vec![],
        }
    }
}

impl<'a> Iterator for NaturalNeighborIterator<'a, EntityPosition> {
    type Item = &'a EntityPosition;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(neighbor) = self.nearest.next() {
            let is_in_polygon = self.polygon_perimeter.iter().all(|line| {
                is_less_than(self.query_point, line) == is_less_than(neighbor.position, line)
            });
            if is_in_polygon {
                let line = Line::bisector(self.query_point, neighbor.position);
                self.polygon_perimeter.push(line);
                return Some(neighbor);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{EntityPosition, IntoNaturalNeighborIterator};
    use spade::rtree::RTree;
    use specs::prelude::*;

    #[test]
    fn natural_neighbor_iterator() {
        let mut world = World::new();
        let mut rtree = RTree::new();

        let entity_positions = vec![
            EntityPosition {
                entity: world.create_entity().build(),
                position: [2.0, 0.0], // nearest and first natural neigbor
            },
            EntityPosition {
                entity: world.create_entity().build(),
                position: [3.0, 0.0], // second nearest but not a natural neighbor
            },
            EntityPosition {
                entity: world.create_entity().build(),
                position: [0.0, 4.0], // third nearest and second natural neighbor
            },
            EntityPosition {
                entity: world.create_entity().build(),
                position: [-1.0, -6.0], // fourth nearest and third natural neighbor
            },
            EntityPosition {
                entity: world.create_entity().build(),
                position: [-2.0, -6.0], // fifth nearest but not a natural neighbor
            },
            EntityPosition {
                entity: world.create_entity().build(),
                position: [-30.0, 1.999], // sixth nearest and fourth natural neighbor
            },
            EntityPosition {
                entity: world.create_entity().build(),
                position: [-31.0, 2.001], // seventh nearest but not a natural neighbor
            },
        ];
        for &epos in entity_positions.iter() {
            rtree.insert(epos);
        }
        let expected: Vec<&EntityPosition> = entity_positions
            .iter()
            .enumerate()
            .filter_map(|(i, epos)| {
                if vec![0, 2, 3, 5].contains(&i) {
                    Some(epos)
                } else {
                    None
                }
            })
            .collect();
        let natural_neighbors: Vec<&EntityPosition> =
            rtree.natural_neighbor_iterator(&[0.0, 0.0]).collect();
        assert_eq!(natural_neighbors, expected);
    }
}
