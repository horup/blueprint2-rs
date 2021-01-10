use std::collections::HashMap;
use image::DynamicImage;
use crate::shared::HashId;

pub trait Assets {
    fn load_texture(&mut self, id:HashId, image:DynamicImage);
} 
