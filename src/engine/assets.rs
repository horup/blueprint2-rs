use std::{collections::HashMap};

use crate::shared::{HashId, RGBAImage, SpriteSheet, log};
use crate::shared::{Assets as AssetsTrait};

use glow::*;

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
                    //let bytes = [255,0,255,255, 255,255,0,255, 255,255,255,255, 255,255,255,255];
                    gl.tex_image_2d(glow::TEXTURE_2D, level, internal_format, 
                        width, height, border, src_format, src_type, pixels);

                    gl.generate_mipmap(glow::TEXTURE_2D);
                }

                log("texture created");
            }
        }
        
    }
}

impl AssetsTrait for Assets {
    fn load_texture(&mut self, id:HashId, image:RGBAImage) -> HashId {
        self.textures.insert(id, (image, TextureKey::default()));
        id
    }
    fn load_spritesheet(&mut self, id:HashId, spritesheet:SpriteSheet) -> HashId {
        self.spritesheets.insert(id, spritesheet);
        id
    }
}