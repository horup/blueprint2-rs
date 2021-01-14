use nalgebra::Vector3;

use crate::{shared::{ArenaItem, Index}};
use super::{Sprite, game::Game};

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Entity<T:Game> {
    index:Index,
    pub pos:Vector3<f32>,
    pub sprite:Sprite,
    pub ext:T::GameEntity
}

impl<T:Game> ArenaItem for Entity<T> {
    fn with_index(self, index:Index) -> Self {
        Self {
            index:index,
            ..self
        }
    }

    fn index(&self) -> Index {
        self.index
    }
}



