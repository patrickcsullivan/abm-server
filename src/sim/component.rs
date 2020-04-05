use specs::{prelude::*, Component};
use specs_derive::Component;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
#[storage(VecStorage)]
pub struct Pos {
    pub x: u8,
    pub y: u8,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
#[storage(VecStorage)]
pub struct Food(pub u8);

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
#[storage(VecStorage)]
pub struct Metabolism(pub u8);

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
#[storage(VecStorage)]
pub struct Vision(pub u8);
