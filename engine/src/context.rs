use super::{Assets, Camera, Event, Game, States};

pub struct Context<'a, T:Game> {
    pub event:Event<T>,
    pub states:&'a mut States<T>,
    pub assets:&'a mut Assets,
    pub camera:&'a mut Camera
}