use crate::{game::{AssetKey, SpriteSheet}, shared::HashId};


/// A component holding sprite information for an entity.
#[derive(Clone, Copy, Default)]
pub struct Sprite {
    pub spritesheet:AssetKey<SpriteSheet>,
    pub frame:u16
}