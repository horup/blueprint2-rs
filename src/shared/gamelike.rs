use super::{Context, Enginelike, Event};
pub trait Gamelike {
    type GameThing;
    type GameEvent;

    fn on_event(&mut self, context:Context);
}