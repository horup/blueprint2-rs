use std::collections::HashMap;

use crate::shared::{HashId, SpriteSheet};
use crate::shared::{Assets as AssetsTrait};

#[derive(Default)]
pub struct Assets  {
    pub textures:HashMap<HashId, i32>
}

impl AssetsTrait for Assets {
    fn load_texture(&mut self, id:&str, image:image::DynamicImage) {

    }
    fn load_spritesheet(&mut self, id:&str, spritesheet:SpriteSheet) {
        
    }
}