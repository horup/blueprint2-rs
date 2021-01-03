use generational_arena::Arena;

use crate::{Event, Mesh, World, draw, log};

pub struct Engine {
    pub world:World,
    pub gl:glow::Context,
    pub width:i32,
    pub height:i32,
    pub meshes:Arena<Mesh>
}

impl Engine {
    pub fn new(gl:glow::Context) -> Self {
        Self {
            world:World::new(),
            gl,
            width:0,
            height:0,
            meshes:Arena::new()
        }
    }

    pub fn setup(&mut self) {
        unsafe {
            let mesh = Mesh::new_quad(&mut self.gl);
            self.meshes.insert(mesh);
        }
    }

    pub fn update(&mut self, e:Event) {
        match e {
            Event::Initialize => {
                self.setup_shaders();
                self.setup();
            }
            Event::Update(_) => {}
            Event::Draw(_) => {
                self.draw();
            }
        }
    }
}
