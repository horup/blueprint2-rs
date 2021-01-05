use generational_arena::Index;
use glow::Context;

use crate::Mesh;

#[derive(Clone, Copy)]
/// An object which encapsulates the state of a sprite.
pub struct Sprite {
    sprite_mesh_id:Option<Index>,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            sprite_mesh_id:None
        }
    }
}


