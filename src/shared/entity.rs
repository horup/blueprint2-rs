use generational_arena::Index;
use nalgebra::Vector3;

use super::{Gamelike, Sprite};

#[derive(Clone, Copy, PartialEq)]
pub struct Entity<T:Gamelike> {
    pub pos:Vector3<f32>,
    pub sprite:Sprite,
    pub ext:T::GameEntity
}

impl<T:Gamelike> Default for Entity<T> {
    fn default() -> Self {
        Self {
            pos:Vector3::default(),
            sprite:Sprite::default(),
            ext:T::GameEntity::default()
        }
    }
}



