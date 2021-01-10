use super::{Event, Game, HashId, States};
use image::DynamicImage;

pub trait Context<T:Game> {
    fn event(&self) -> Event;
    fn states(&mut self) -> &mut States<T>;
}