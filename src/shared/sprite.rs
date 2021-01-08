use super::{Gamelike};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// An object which encapsulates the state of a sprite.
pub struct Sprite<T:Gamelike> {
    pub texture:T::Texture
}

impl<T:Gamelike> Default for Sprite<T> {
    fn default() -> Self {
        Self {
            texture:T::Texture::default()
        }
    }
}


