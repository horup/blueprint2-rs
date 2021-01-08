
use super::{Enginelike, Event, State};

pub struct Context<'a> {
    pub current:&'a mut State,
    pub previous:&'a mut State,
    pub event:Event
}

