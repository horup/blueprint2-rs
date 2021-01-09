use std::collections::HashMap;

use crate::shared::HashId;

#[derive(Default)]
pub struct Assets {
    textures:HashMap<HashId, i32>,
    sprite_sheets:HashMap<HashId, i32>
}