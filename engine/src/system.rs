use crate::Game;
use crate::Context;
pub trait System<S:Game> {
    fn update(&mut self, context:&mut Context<S>) {
    }
}