use super::{HashId};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// An object which encapsulates the state of a sprite.
pub struct Sprite {
    pub spritesheet:HashId,
    pub frame:u32
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            spritesheet:HashId::default(),
            frame:0
        }
    }
}


