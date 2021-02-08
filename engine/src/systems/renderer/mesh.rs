use core::slice;

use glow::{Context, HasContext, WebBufferKey, WebProgramKey, WebVertexArrayKey};
use web_sys::{WebGlVertexArrayObject};

use super::{Camera, Vertex};

pub struct Mesh {
    pub vertices:Vec<Vertex>,
    pub vertex_buffer_object:WebBufferKey,
    pub vertex_array_object:WebVertexArrayKey
}


impl Mesh {
    pub unsafe fn new_quads(gl:&Context, count:usize) -> Self {
        let mut vertices = Vec::new();
        for i in 0..count {
            // lower right triangle
            vertices.push(Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0)); //1
            vertices.push(Vertex::new(0.5, -0.5, 0.0, 1.0, 0.0)); //2
            vertices.push(Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0)); //3

            // upper left triangle
            vertices.push(Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0)); //1
            vertices.push(Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0)); //3
            vertices.push(Vertex::new(-0.5, 0.5, 0.0, 0.0, 1.0)); //4
        }

        return Self::new(&vertices, gl);
    }

    pub unsafe fn new(vertices:&[Vertex], gl:&Context) -> Self {
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
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vertex_buffer_object));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &buffer, glow::DYNAMIC_DRAW);
    }

    pub unsafe fn draw(&self, gl:&Context, program:WebProgramKey, camera:&Camera) {
        self.draw_subset(gl, program, self.vertices.len(), camera);
    }

    
    /// Draws a subset of the mesh where `count` is the number of vertices to draw
    pub unsafe fn draw_subset(&self, gl:&Context, program:WebProgramKey, count:usize, camera:&Camera) {
        let projection = gl.get_uniform_location(program, "projection");
        let view = gl.get_uniform_location(program, "view");
        if let (Some(view), Some(projection)) = (view, projection) {
            gl.uniform_matrix_4_f32_slice(Some(&view), false, camera.view.to_homogeneous().as_slice());
            gl.uniform_matrix_4_f32_slice(Some(&projection), false, camera.projection.as_slice());
            gl.bind_vertex_array(Some(self.vertex_array_object));
            let mut count = count;
            if count > self.vertices.len() {
                count = self.vertices.len();
            }
            gl.draw_arrays(glow::TRIANGLES, 0, count as i32);
        }

    }

    /// Copies the vertices storied in `vertices` into the mesh at `dest_index`
    /// Note: does not grow the underlying buffer, vertices outside the buffer will be ignored!
    pub fn copy_from(&mut self, vertices:&[Vertex], dest_index:usize) {
        let mut i = dest_index;
        for source in vertices {
            if let Some(target) = self.vertices.get_mut(i) {
                *target = *source;
            } 
            i += 1;
        }
    }

}