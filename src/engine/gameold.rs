use std::hash::Hash;

use super::Context;

pub trait GameOld : Default {
    type GameEntity : Default + Copy + Clone + Eq + PartialEq + Hash;
    type GameEvent : Copy + Clone + Eq + PartialEq + Hash;
    fn update(&mut self, context:&mut Context<Self>);
    fn initialize(&mut self, context:&mut Context<Self>) {
        
    } 
}