use std::rc::Rc;

use glow::{HasContext, TEXTURE_MIN_FILTER, WebTextureKey};

use super::Frame;
pub type TextureKey = WebTextureKey;

pub struct RGBAImage {
    pub width:u32,
    pub height:u32,
    pub pixels:Vec<u8>,
    pub texture:TextureKey
}

impl Default for RGBAImage {
    fn default() -> Self {

        let mut default_pixels = [
            255,0,0,255,
            0,0,255,255,
            0,0,255,255,
            255,0,0,255
        ];


        Self {
            height:2,
            width:2,
            pixels:default_pixels.into(),
            texture:TextureKey::default()
        }
    }
}

// TODO: implement drop to free texture
impl RGBAImage {
    pub fn flip(self) -> Self
    {
        let mut pixels = self.pixels;
        let len = pixels.len() / 2;
        let (top, bottom) = pixels.split_at_mut(len);
        assert!(top.len() == bottom.len());

        let width = self.width as usize;
        let height = self.height as usize / 2 as usize;
        let row_size = width * 4 as usize;
        let mut temp = vec![0; row_size];

        for row1 in 0..height {
            let row2 = height - row1 -1;
            let row1 = row1 * row_size .. (row1 + 1) * row_size;
            let row2 = row2 * row_size .. (row2 + 1) * row_size;
            
            // copy row1 to temp
            temp.copy_from_slice(&top[row1.clone()]);

            // copy row2 to row1
            top[row1.clone()].copy_from_slice(&bottom[row2.clone()]);

            // copy temp to row2
            bottom[row2.clone()].copy_from_slice(&temp);
           
        }



        RGBAImage {
            width:self.width,
            height:self.height,
            pixels:pixels,
            texture:TextureKey::default()
        }
    }

    pub unsafe fn load_texture(&mut self, gl:Rc<glow::Context>) {
        if self.texture == TextureKey::default() {
            self.texture = gl.create_texture().unwrap();;
            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);

            let level = 0;
            let internal_format = glow::RGBA as i32;
            let width = self.width as i32;
            let height = self.height as i32;
            let border = 0;
            let src_format = glow::RGBA;
            let src_type = glow::UNSIGNED_BYTE;
            let pixels = Some(self.pixels.as_slice());
            gl.tex_image_2d(glow::TEXTURE_2D, level, internal_format, 
                width, height, border, src_format, src_type, pixels);

            gl.generate_mipmap(glow::TEXTURE_2D);
        }
    }

    /// Constructs a `Frame` with normalized coordinates based upon dimensions of `RGBAImage`
    /// `x`,`y` are screen coordinates with y pointing down. 
    /// Returned `Frame` have coordinates in opengl texture space, i.e. with positive y axis pointing up!
    pub fn frame(&self, x:u32, y:u32, width:u32, height:u32) -> Frame {
        let w = width as f32 / self.width as f32;
        let h = height as f32/ self.height as f32;
        let u = x as f32 / self.height as f32;
        let v = (1.0 - h) - y as f32 / self.height as f32;

        Frame {
            u,
            v,
            w,
            h,
        }
    }
}
