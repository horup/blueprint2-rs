use crate::{Event, Game};
use crate::Context;
pub trait System<S:Game> {
    fn update(&mut self, event:&Event<S>, context:&mut Context<S>) {
    }
}