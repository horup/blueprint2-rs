use std::{collections::HashMap, io::Cursor};
use glow::*;
use image::ImageDecoder;
use crate::shared::{HashId, log};
use super::{RGBAImage, SpriteSheet};
type TextureKey = WebTextureKey;

#[derive(Default)]
pub struct Assets  {
    pub textures:HashMap<HashId, (RGBAImage, TextureKey)>,
    pub spritesheets:HashMap<HashId, SpriteSheet>
}

impl Assets {
    pub fn update(&mut self, gl:&glow::Context) {
        for (id, (img, tex_id)) in self.textures.iter_mut() {
            if *tex_id == TextureKey::default() {
                unsafe {
                    let texture = gl.create_texture().unwrap();
                    *tex_id = texture;
                    gl.bind_texture(glow::TEXTURE_2D, Some(*tex_id));

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

    pub fn load_texture_from_png_bytes(&mut self, id:HashId, bytes:&[u8]) -> &RGBAImage {
        let cursor = Cursor::new(bytes);
        let res = image::png::PngDecoder::new(cursor).expect("image was not of png");
        let (w, h) = res.dimensions();
        let mut buffer = vec![0; res.total_bytes() as usize];
        buffer.as_mut_slice();
        res.read_image(&mut buffer);

        &self.load_texture(id,  RGBAImage {
            width:w,
            height:h,
            pixels:buffer
        }.flip())
    }

    pub fn load_texture(&mut self, id:HashId, image:RGBAImage) -> &RGBAImage {
        self.textures.insert(id, (image, TextureKey::default()));
        &self.textures.get(&id).unwrap().0
    }
    
    pub fn load_spritesheet(&mut self, id:HashId, spritesheet:SpriteSheet) -> HashId {
        self.spritesheets.insert(id, spritesheet);
        id
    }
}