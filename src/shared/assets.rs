use std::{collections::{HashMap}, io::Cursor, usize, vec};
use image::DynamicImage;
use crate::shared::HashId;
use super::{SpriteSheet, log};
use image::*;

pub struct RGBAImage {
    pub width:u32,
    pub height:u32,
    pub pixels:Vec<u8>
}

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
            pixels:pixels
        }
    }
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
        }.flip())
    }
    fn load_spritesheet(&mut self, id:HashId, spritesheet:SpriteSheet) -> HashId;
} 
