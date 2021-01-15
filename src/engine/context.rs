use super::{Assets, Event, States, gameold::GameOld};

pub struct Context<'a, T:GameOld> {
    pub event:Event,
    pub states:&'a mut States<T>,
    pub assets:&'a mut Assets
}