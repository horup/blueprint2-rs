use crate::game::{Context, Game};

pub trait System<S:Game> {
    fn update(&mut self, context:&mut Context<S>) {
        // nop
    }
}