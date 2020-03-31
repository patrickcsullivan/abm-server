use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BoundingBox {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

// impl BoundingBox {
//     pub fn new(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> BoundingBox {
//         return BoundingBox {
//             min_x: min_x,
//             max_x: max_x,
//             min_y: min_y,
//             max_y: max_y,
//         };
//     }
// }
