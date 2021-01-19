use std::{collections::HashMap, io::Cursor};
use glow::*;
use image::ImageDecoder;
use crate::shared::{HashId, log};

use super::{AssetCollection, AssetKey, RGBAImage, SpriteSheet, TextureKey};

#[derive(Default)]
pub struct Assets  {
    pub textures:AssetCollection<RGBAImage>,
    pub spritesheets:AssetCollection<SpriteSheet>
}

impl Assets {
    pub fn update(&mut self, gl:&glow::Context) {
        for (id, img) in self.textures.iter_mut() {
            if img.texture == TextureKey::default() {
                unsafe {
                    let texture = gl.create_texture().unwrap();
                    img.texture = texture;
                    gl.bind_texture(glow::TEXTURE_2D, Some(img.texture));

                    let level = 0;
                    let internal_format = glow::RGBA as i32;
                    let width = img.width as i32;
                    let height = img.height as i32;
                    log(&format!("{}", width));
                    let border = 0;
                    let src_format = glow::RGBA;
                    let src_type = glow::UNSIGNED_BYTE;
                    let pixels = Some(img.pixels.as_slice());
                    gl.tex_image_2d(glow::TEXTURE_2D, level, internal_format, 
                        width, height, border, src_format, src_type, pixels);

                    gl.generate_mipmap(glow::TEXTURE_2D);
                }
                log("texture created");
            }
        }
        
    }

    pub fn load_texture_from_png_bytes(&mut self, id:AssetKey<RGBAImage>, bytes:&[u8]) -> &RGBAImage {
        let cursor = Cursor::new(bytes);
        let res = image::png::PngDecoder::new(cursor).expect("image was not of png");
        let (w, h) = res.dimensions();
        let mut buffer = vec![0; res.total_bytes() as usize];
        buffer.as_mut_slice();
        res.read_image(&mut buffer);

        &self.load_texture(id,  RGBAImage {
            width:w,
            height:h,
            pixels:buffer,
            texture:TextureKey::default()
        }.flip())
    }

    pub fn load_texture(&mut self, id:AssetKey<RGBAImage>, image:RGBAImage) -> &RGBAImage {
        self.textures.insert(id, image);
        &self.textures.get(&id).unwrap()
    }
    
    pub fn load_spritesheet(&mut self, id:AssetKey<SpriteSheet>, spritesheet:SpriteSheet) -> AssetKey<SpriteSheet> {
        self.spritesheets.insert(id, spritesheet);
        id
    }
}