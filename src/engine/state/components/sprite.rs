use crate::shared::HashId;


/// A component holding sprite information for an entity.
#[derive(Debug, Clone, Copy, Default)]
pub struct Sprite {
    pub spritesheet:HashId,
    pub frame:u16
}