use super::{Context, Event};
pub trait Gamelike {
    type GameThing;
    type GameEvent;

    fn update(&mut self, context:Context);
}