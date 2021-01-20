use std::default;

use crate::shared::HashId;

use super::{AssetKey, RGBAImage};

#[derive(Clone, Copy, PartialEq)]
pub struct Frame {
    pub u:f32,
    pub v:f32,
    pub w:f32,
    pub h:f32
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            u:0.0, 
            v:0.0, 
            w:1.0,
            h:1.0
        }
    }
}
/*
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Animation {
    Default,
    None,
    LoopForwardBackward,
    LoopBackwardForward,
    LoopReset,
    ForwardStop,
    Stopped
}*/

pub struct SpriteSheet {
    pub texture:AssetKey<RGBAImage>,
    pub frames:Vec<Frame>
}

impl Default for SpriteSheet {
    fn default() -> Self {
        Self {
            texture:AssetKey::default(),
            frames:[Frame {u:0.0, v:0.0, w:1.0, h:1.0}].into()
        }
    }
}

impl SpriteSheet {
    /// Constructs a new `SpriteSheet` with a single frame spanning the whole `texture`
    pub fn new(texture:AssetKey<RGBAImage>,) -> Self {
        Self::new_1x1(texture, Frame::default())
    }

    /// Constructs a new `SpriteSheet` with a single `frame`
    pub fn new_1x1(texture:AssetKey<RGBAImage>, frame:Frame) -> Self {
        Self {
            texture,
            frames:[frame].into()
        }
    }

    pub fn get_frame(&self, index:usize) -> Frame {
        if self.frames.len() == 0 {
            return Frame {u:0.0, v:0.0, w:1.0, h:1.0};
        }
        
        *self.frames.get(index % self.frames.len()).unwrap()
    }
}