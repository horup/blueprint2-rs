use super::{Event, World};

pub struct Context<'a> {
    pub current:&'a mut World,
    pub event:Event
}