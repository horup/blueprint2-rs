use std::collections::HashMap;
use image::DynamicImage;
use crate::shared::HashId;
use super::SpriteSheet;


pub trait Assets {
    fn load_texture(&mut self, id:HashId, image:DynamicImage) -> HashId;
    fn load_texture_from_bytes(&mut self, id:HashId, bytes:&[u8]) -> HashId {
        let img = image::load_from_memory(bytes).unwrap();
        self.load_texture(id.into(), img);
        id
    }
    fn load_spritesheet(&mut self, id:HashId, spritesheet:SpriteSheet) -> HashId;
} 
