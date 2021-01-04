use generational_arena::Index;
use nalgebra::Vector3;

use crate::{Engine, Sprite};

#[derive(Clone, Copy, Default)]
pub struct Thing {
    pub pos:Vector3<f32>,
    pub sprite:Sprite
}

impl Thing {
    pub fn update(&mut self, engine:&mut Engine) {
        
    }
}



