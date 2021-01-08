use super::{Context, Enginelike, Event};
pub trait Gamelike {
    type GameThing;
    type GameEvent;

    fn update(&mut self, context:Context);
}