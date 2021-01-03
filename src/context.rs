use crate::{Event, World};

pub struct Context<'a> {
    pub world:&'a mut World,
    pub event:Event
}
