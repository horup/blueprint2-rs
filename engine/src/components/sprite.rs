use crate::{AssetKey, SpriteSheet};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum AnimationState {
    None,
    Loop
}

impl Default for AnimationState {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Animation {
    pub state:AnimationState,
    pub speed:f32
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            state:AnimationState::None,
            speed:1.0
        }
    }
}


/// A component holding sprite information for an entity.
#[derive(Clone, Copy, Default)]
pub struct Sprite {
    pub spritesheet:AssetKey<SpriteSheet>,
    pub frame:f32,
    pub animation:Animation
}