use std::{collections::{HashMap}, io::Cursor, vec};
use image::DynamicImage;
use crate::shared::HashId;
use super::SpriteSheet;
use image::*;

pub struct RGBAImage {
    pub width:u32,
    pub height:u32,
    pub pixels:Vec<u8>
}

pub trait Assets {
    fn load_texture(&mut self, id:HashId, image:RGBAImage) -> HashId;
    fn load_texture_from_png_bytes(&mut self, id:HashId, bytes:&[u8]) -> HashId {
        let cursor = Cursor::new(bytes);
        let res = image::png::PngDecoder::new(cursor).expect("image was not of png");
        let (w, h) = res.dimensions();
        let mut buffer = vec![0; res.total_bytes() as usize];
        buffer.as_mut_slice();
        res.read_image(&mut buffer);
  
        self.load_texture(id,  RGBAImage {
            width:w,
            height:h,
            pixels:buffer
        })
    }
    fn load_spritesheet(&mut self, id:HashId, spritesheet:SpriteSheet) -> HashId;
} 
