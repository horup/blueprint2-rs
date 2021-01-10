use std::collections::HashMap;

use crate::shared::HashId;
use crate::shared::{Assets as AssetsTrait};

#[derive(Default)]
pub struct Assets  {
    pub textures:HashMap<HashId, i32>
}

impl AssetsTrait for Assets {
    fn load_texture(&mut self, id:HashId, image:image::DynamicImage) {
        
    }
}