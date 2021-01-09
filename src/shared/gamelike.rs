use std::hash::Hash;

use super::{Context, Event};
pub trait Gamelike : Default {
    type GameEntity : Default + Copy + Clone + Eq + PartialEq + Hash;
    type GameEvent;
    fn update(&mut self, context:Context);
}