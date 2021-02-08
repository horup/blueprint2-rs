use std::{collections::HashMap,  io::Cursor, rc::Rc};
use glow::*;
use image::ImageDecoder;

use super::{AssetCollection, AssetKey, RGBAImage, SpriteSheet, TextureKey};

pub struct Assets  {
    pub textures:AssetCollection<RGBAImage>,
    pub spritesheets:AssetCollection<SpriteSheet>,
    pub gl:Rc<glow::Context>
}

impl Assets {
    pub fn new(gl:Rc<glow::Context>) -> Self {
        Self {
            gl:gl,
            spritesheets:AssetCollection::default(),
            textures:AssetCollection::default()
        }
    }
    pub fn update(&mut self) {
        for (id, img) in self.textures.iter_mut() {
            if img.texture == TextureKey::default() {
                unsafe {
                    img.load_texture(self.gl.clone());
                }
            }
        }
        
    }

    pub fn load_texture_from_png_bytes(&mut self, key:AssetKey<RGBAImage>, bytes:&[u8]) -> &RGBAImage {
        let cursor = Cursor::new(bytes);
        let res = image::png::PngDecoder::new(cursor).expect("image was not of png");
        let (w, h) = res.dimensions();
        let mut buffer = vec![0; res.total_bytes() as usize];
        buffer.as_mut_slice();
        res.read_image(&mut buffer);

        let tex = RGBAImage {
            width:w,
            height:h,
            pixels:buffer,
            texture:TextureKey::default()
        }.flip();

        self.textures.insert(key, tex);
        self.textures.get(&key)
    }
    
    pub fn load_spritesheet(&mut self, key:AssetKey<SpriteSheet>, spritesheet:SpriteSheet) -> AssetKey<SpriteSheet> {
        self.spritesheets.insert(key, spritesheet);
        key
    }
}