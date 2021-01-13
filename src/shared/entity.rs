use nalgebra::Vector3;

use super::{Game, Sprite};

#[derive(Clone, Copy, PartialEq)]
pub struct Entity<T:Game> {
    pub pos:Vector3<f32>,
    pub sprite:Sprite,
    pub ext:T::GameEntity
}

impl<T:Game> Default for Entity<T> {
    fn default() -> Self {
        Self {
            pos:Vector3::default(),
            sprite:Sprite::default(),
            ext:T::GameEntity::default()
        }
    }
}



