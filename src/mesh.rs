use glow::{Context, HasContext, WebBufferKey, WebVertexArrayKey};
use web_sys::WebGlVertexArrayObject;

use crate::log;

use super::Vertex;

pub struct Mesh {
    pub vertices:Vec<Vertex>,
    pub vertex_buffer_object:WebBufferKey,
    pub vertex_array_object:WebVertexArrayKey
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

    pub unsafe fn new(vertices:&[Vertex], gl:&mut Context) -> Self {
        let vertex_array_object = gl.create_vertex_array().expect("cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array_object));
        let vertex_buffer_object = gl.create_buffer().expect("failed");
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer_object));

        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, std::mem::size_of::<Vertex>() as i32, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, std::mem::size_of::<Vertex>() as i32, (std::mem::size_of::<u32>() * 3) as i32);
        gl.enable_vertex_attrib_array(1);

        let mut mesh = Self { 
            vertices:vertices.to_vec(),
            vertex_array_object,
            vertex_buffer_object
        };

        mesh.update(gl);

        return mesh;
    }

    pub unsafe fn update(&self, gl:&Context) {
        let buffer = std::slice::from_raw_parts(self.vertices.as_ptr() as *const u8, self.vertices.len() * std::mem::size_of::<Vertex>());
        let buffer = buffer.align_to().1;        
        //log(&format!("{:?}", buffer));

        gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vertex_buffer_object));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &buffer, glow::DYNAMIC_DRAW);
    }

    pub unsafe fn draw(&self, gl:&Context) {
        gl.bind_vertex_array(Some(self.vertex_array_object));
        gl.draw_arrays(glow::TRIANGLES, 0, self.vertices.len() as i32);
    }
}