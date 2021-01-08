use std::hash::Hash;

use super::{Context, Event};
pub trait Gamelike {
    type GameEntity : Default + Copy + Clone + Eq + PartialEq + Hash;
    type GameEvent;
    type Texture : Default + Copy + Clone + Eq + PartialEq + Hash;
    type Spritesheet;

    fn update(&mut self, context:Context);
}