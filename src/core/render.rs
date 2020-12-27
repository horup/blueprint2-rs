use generational_arena::Arena;
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
        let vertex_shader_source = include_str!("./shaders/default.vert");
        let fragment_shader_source = include_str!("./shaders/default.frag");

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

      /*  let pos_loc = gl.get_attrib_location(program, "pos").expect("get_attrib_location failed");
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, std::mem::size_of::<Vertex>() as i32, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, std::mem::size_of::<Vertex>() as i32, (std::mem::size_of::<u32>() * 3) as i32);
        gl.enable_vertex_attrib_array(1);*/

        gl.use_program(Some(program));
    }

    unsafe fn setup_buffers(&mut self) {
        /*let vertex_array = self.gl.create_vertex_array().expect("cannot create vertex array");
        self.gl.bind_vertex_array(Some(vertex_array));

        let position_buffer = self.gl.create_buffer().expect("failed");
        self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(position_buffer));
            
        let mut vertices = Vec::new();
        // lower right triangle
        vertices.push(Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0)); //1
        vertices.push(Vertex::new(0.5, -0.5, 0.0, 1.0, 0.0)); //2
        vertices.push(Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0)); //3

        // upper left triangle
        vertices.push(Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0)); //1
        vertices.push(Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0)); //3
        vertices.push(Vertex::new(-0.5, 0.5, 0.0, 0.0, 1.0)); //4



        let buffer = std::slice::from_raw_parts(vertices.as_ptr() as *const u8, vertices.len() * std::mem::size_of::<Vertex>());
        let buffer = vertices.align_to().1;        
        log(&format!("{:?}", buffer));

        self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &buffer, glow::DYNAMIC_DRAW);*/
    }

    pub fn new(gl:Context) -> Self {
        let mut render = Render {
            gl:gl,
            width:1,
            height:1,
            meshes:Arena::new()
        };
        unsafe {
            render.setup_buffers();
            render.setup_shaders();
        }
        return render;
    }

    pub fn insert_quad(&mut self) {
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
            self.meshes.insert(mesh);
        }
    }
    
    pub fn draw(&mut self) {
        let gl = &mut self.gl;
        unsafe {
            gl.viewport(0, 0, self.width, self.height);
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            

            for (_, mesh) in &self.meshes {
                mesh.draw(gl);
            }

            let count = 1;
            
           // gl.draw_arrays(glow::TRIANGLES, 0, 6 * count);
        }
    }
}