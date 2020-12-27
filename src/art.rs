use super::GameWorld;
use nalgebra::Vector2;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Animation {
    Default,
    None,
    LoopForwardBackward,
    LoopBackwardForward,
    LoopReset,
    ForwardStop,
    Stopped
}
impl Default for Animation {
    fn default() -> Self {
        Animation::None
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Frame {
    pub x:f32,
    pub y:f32,
    pub w:f32,
    pub h:f32
}

impl Frame {
    pub fn new(x:f32, y:f32, w:f32, h:f32) -> Self {
        Self {x, y, w, h}
    }
}

// TODO: refactor Rect2 into Frame type, since this will be extended in the future.
// TODO: add new_xxx which can instantiate zombie game
#[derive(Clone, Default)]
pub struct Art<W:GameWorld> {
    pub texture:W::Texture,
    pub frames:Vec<Frame>,
    pub frames_per_second:f32,
    pub default_animation:Animation,
    pub origin:Vector2<f32>
}

impl<W:GameWorld> Art<W> {
    pub fn new_1x1(texture:W::Texture, frame:Frame) -> Art<W> {
        Self {
            texture:texture,
            frames:[frame].into(),
            default_animation:Animation::None,
            frames_per_second:0.0,
            origin:Vector2::new(0.5, 0.5)
        }
    }
}