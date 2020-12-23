use glow::{Context, HasContext, WebBufferKey};

use super::Vertex;

pub struct Mesh {
    pub vertices:Vec<Vertex>,
    pub buffer:WebBufferKey
}

impl Mesh {
    pub unsafe fn new(vertices:&[Vertex], gl:&mut Context) -> Self {
        let mesh = Self { 
            vertices:vertices.to_vec(),
            buffer:gl.create_buffer().expect("failed")
        };



        return mesh;
    }

    pub unsafe fn update(&mut self) {
        let buffer = std::slice::from_raw_parts(self.vertices.as_ptr() as *const u8, self.vertices.len() * std::mem::size_of::<Vertex>());
        
    }
}