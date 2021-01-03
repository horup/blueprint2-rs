use generational_arena::{Arena, Index};
use glow::*;
use wasm_bindgen::prelude::*;

use crate::log;

use super::{Mesh, Vertex};

pub struct Render {
    pub gl:Context,
    pub width:i32,
    pub height:i32,
    meshes:Arena<Mesh>
}

// TODO: fixed aspect
impl Render {
    unsafe fn setup_shaders(&mut self) {
        let gl = &mut self.gl;
        let vertex_shader_source = include_str!("../shaders/default.vert");
        let fragment_shader_source = include_str!("../shaders/default.frag");

        let program = gl.create_program().expect("Cannot create program");
    
        let shader = gl.create_shader(glow::VERTEX_SHADER).expect("could not create shader");
        gl.shader_source(shader, vertex_shader_source);
        gl.compile_shader(shader);
        gl.attach_shader(program, shader);

        let shader = gl.create_shader(glow::FRAGMENT_SHADER).expect("could not create shader");
        gl.shader_source(shader, fragment_shader_source);
        gl.compile_shader(shader);
        gl.attach_shader(program, shader);

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!(gl.get_program_info_log(program));
        }

        gl.use_program(Some(program));
    }

    pub fn new(gl:Context) -> Self {
        let mut render = Render {
            gl:gl,
            width:1,
            height:1,
            meshes:Arena::new()
        };
        unsafe {
            render.setup_shaders();
        }
        return render;
    }

    pub fn insert_quad(&mut self) -> Index {
        unsafe {
            let mut vertices = Vec::new();
            // lower right triangle
            vertices.push(Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0)); //1
            vertices.push(Vertex::new(0.5, -0.5, 0.0, 1.0, 0.0)); //2
            vertices.push(Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0)); //3

            // upper left triangle
            vertices.push(Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0)); //1
            vertices.push(Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0)); //3
            vertices.push(Vertex::new(-0.5, 0.5, 0.0, 0.0, 1.0)); //4

            
            let mesh = Mesh::new(&vertices, &mut self.gl);
            self.meshes.insert(mesh)
        }
    }

    pub fn get_mesh(&self, index:Index) -> Option<&Mesh> {
        self.meshes.get(index)
    }

    pub fn get_mesh_mut(&mut self, index:Index) -> Option<&mut Mesh> {
        self.meshes.get_mut(index)
    }
    
    pub fn draw(&mut self) {
        let gl = &mut self.gl;
        unsafe {
            gl.viewport(0, 0, self.width, self.height);
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            for (_, mesh) in &self.meshes {
                gl.bind_vertex_array(Some(mesh.vertex_array_object));
                gl.draw_arrays(glow::TRIANGLES, 0, mesh.vertices.len() as i32);
            }
        }
    }
}