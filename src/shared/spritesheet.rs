use std::default;

use super::HashId;

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
    pub texture:HashId,
    pub frames:Vec<Frame>
}

impl SpriteSheet {
    /// Constructs a new `SpriteSheet` with a single frame spanning the whole `texture`
    pub fn new(texture:HashId) -> Self {
        Self::new_1x1(texture, Frame::default())
    }

    /// Constructs a new `SpriteSheet` with a single `frame`
    pub fn new_1x1(texture:HashId, frame:Frame) -> Self {
        Self {
            texture,
            frames:[frame].into()
        }
    }
}