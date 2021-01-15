use super::{Assets, Event, States, game::Game};

pub struct Context<'a, T:Game> {
    pub event:Event,
    pub states:&'a mut States<T>,
    pub assets:&'a mut Assets
}