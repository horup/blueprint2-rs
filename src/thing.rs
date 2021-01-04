use generational_arena::Index;
use nalgebra::Vector3;

use crate::Sprite;

#[derive(Clone, Copy, Default)]
pub struct Thing {
    pub id:Option<Index>,
    pub pos:Vector3<f32>,
    pub sprite:Sprite
}

impl Thing {
    
}



