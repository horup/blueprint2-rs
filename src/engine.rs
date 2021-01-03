use crate::{Event, World, draw, setup_shaders};

pub struct Engine {
    pub world:World,
    pub gl:glow::Context,
    pub width:i32,
    pub height:i32
}

impl Engine {
    pub fn new(gl:glow::Context) -> Self {
        Self {
            world:World::new(),
            gl,
            width:0,
            height:0
        }
    }

    pub fn update(&mut self, e:Event) {
        match e {
            Event::Initialize => {
                setup_shaders(self);
            }
            Event::Update(_) => {}
            Event::Draw(_) => {
                draw(self);
            }
        }
    }
}
