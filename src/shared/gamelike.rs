use std::hash::Hash;

use super::{Enginelike, Event};
pub trait Gamelike : Default {
    type GameEntity : Default + Copy + Clone + Eq + PartialEq + Hash;
    type GameEvent : Copy + Clone + Eq + PartialEq + Hash;
    fn update(&mut self, context:&mut dyn Enginelike);
    fn initialize(&mut self, context:&mut dyn Enginelike) {
        
    } 
}