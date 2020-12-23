use glow::{Context, HasContext, WebBufferKey};

use crate::log;

use super::Vertex;

pub struct Mesh {
    pub vertices:Vec<Vertex>,
    pub buffer:WebBufferKey
}

impl Mesh {

    pub unsafe fn new_quad(gl:&mut Context) -> Self {
        let mut vertices = Vec::new();
        // lower right triangle
        vertices.push(Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0)); //1
        vertices.push(Vertex::new(0.5, -0.5, 0.0, 1.0, 0.0)); //2
        vertices.push(Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0)); //3

        // upper left triangle
        vertices.push(Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0)); //1
        vertices.push(Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0)); //3
        vertices.push(Vertex::new(-0.5, 0.5, 0.0, 0.0, 1.0)); //4

        return Self::new(&vertices, gl);
    }
    pub fn new(vertices:&[Vertex], gl:&mut Context) -> Self {
        unsafe {
            let mesh = Self { 
                vertices:vertices.to_vec(),
                buffer:gl.create_buffer().expect("failed")
            };

            return mesh;
        }
    }

    pub fn update(&mut self, gl:Context) {
        unsafe {
            let buffer = std::slice::from_raw_parts(self.vertices.as_ptr() as *const u8, self.vertices.len() * std::mem::size_of::<Vertex>());
            let buffer = buffer.align_to().1;        
            log(&format!("{:?}", buffer));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &buffer, glow::DYNAMIC_DRAW);
        }
    }
}