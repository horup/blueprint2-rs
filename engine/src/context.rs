use crate::System;

use super::{Assets, Camera, Event, Game, States};

pub struct Context<'a, T:Game> {
    pub states:&'a mut States<T>,
    pub assets:&'a mut Assets,
    pub camera:&'a mut Camera
}