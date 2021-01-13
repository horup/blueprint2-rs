use nalgebra::Vector3;

use super::{ArenaItem, Game, Index, Sprite};

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

    fn index(&self) -> super::Index {
        self.index
    }
}



