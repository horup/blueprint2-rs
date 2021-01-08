use super::Texture;

pub struct Frame {
    pub u:f32,
    pub v:f32,
    pub w:f32,
    pub h:f32
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
    pub texture:Texture,
    pub frames:Vec<Frame>
}

impl SpriteSheet {
   
}