use generational_arena::Index;
use nalgebra::Vector3;

use super::Sprite;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Thing {
    pub pos:Vector3<f32>,
    pub sprite:Sprite
}



