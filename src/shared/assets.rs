use std::collections::HashMap;
use image::DynamicImage;
use crate::shared::HashId;
use super::SpriteSheet;


pub trait Assets {
    fn load_texture(&mut self, id:&str, image:DynamicImage);
    fn load_texture_from_bytes(&mut self, id:&str, bytes:&[u8]) {
        let img = image::load_from_memory(bytes).unwrap();
        self.load_texture(id.into(), img);
    }
    fn load_spritesheet(&mut self, id:&str, spritesheet:SpriteSheet);
} 
