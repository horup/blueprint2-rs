use super::Texture;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// An object which encapsulates the state of a sprite.
pub struct Sprite {
    pub texture:Texture
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            texture:Texture::None
        }
    }
}


