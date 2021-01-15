use crate::shared::HashId;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// An object which encapsulates the state of a sprite.
pub struct SpriteOld {
    pub spritesheet:HashId,
    pub frame:u32
}

impl Default for SpriteOld {
    fn default() -> Self {
        Self {
            spritesheet:HashId::default(),
            frame:0
        }
    }
}


