use crate::shared::{Enginelike, Event};
pub struct Context {
    pub event:Event
}

impl Enginelike for Context {
    fn event(&self) -> Event {
        self.event
    }
}


