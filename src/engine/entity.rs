use nalgebra::Vector3;

use crate::{shared::{ArenaItem, Index}};
use super::{SpriteOld, gameold::GameOld};

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Entity<T:GameOld> {
    index:Index<Self>,
    pub pos:Vector3<f32>,
    pub sprite:SpriteOld,
    pub ext:T::GameEntity
}

impl<T:GameOld> ArenaItem for Entity<T> {
    fn with_index(self, index:Index<Self>) -> Self {
        Self {
            index:index,
            ..self
        }
    }

    fn index(&self) -> Index<Self> {
        self.index
    }
}



