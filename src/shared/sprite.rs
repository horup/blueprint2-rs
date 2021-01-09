use super::{HashId};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// An object which encapsulates the state of a sprite.
pub struct Sprite {
    pub texture:HashId
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            texture:HashId::default()
        }
    }
}


