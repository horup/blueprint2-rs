use std::{collections::HashMap};

use image::DynamicImage;

use crate::shared::{HashId, SpriteSheet};
use crate::shared::{Assets as AssetsTrait};

#[derive(Default)]
pub struct Assets  {
    pub textures:HashMap<HashId, (DynamicImage,i32)>,
    pub spritesheets:HashMap<HashId, SpriteSheet>
}

impl Assets {
    pub fn update(&mut self, gl:&mut glow::Context) {
    }
}

impl AssetsTrait for Assets {
    fn load_texture(&mut self, id:HashId, image:DynamicImage) -> HashId {
        self.textures.insert(id, (image, 0));
        id
    }
    fn load_spritesheet(&mut self, id:HashId, spritesheet:SpriteSheet) -> HashId {
        self.spritesheets.insert(id, spritesheet);
        id
    }
}