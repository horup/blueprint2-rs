use super::{Assets, Event, Game, HashId, States};
use image::DynamicImage;
/*
pub trait Context<T:Game> {
    fn event(&self) -> Event;
    fn states(&mut self) -> &mut States<T>;
    fn assets_mut(&mut self) -> &mut dyn Assets;
}*/

pub struct Context<'a, T:Game> {
    pub event:Event,
    pub states:&'a mut States<T>,
    pub assets:&'a mut dyn Assets
}