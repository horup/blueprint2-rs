use super::{Enginelike, Event};
pub trait Gamelike {
    type GameThing;
    type GameEvent;

    fn on_event(&mut self, engine:&mut impl Enginelike, event:Event);
}