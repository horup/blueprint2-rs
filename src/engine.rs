use generational_arena::Arena;
use crate::{Event, Mesh, World, log};
use glow::*;

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

    pub fn setup_shaders(&mut self) {
        unsafe {
    
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
    
          /*  let pos_loc = gl.get_attrib_location(program, "pos").expect("get_attrib_location failed");
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, std::mem::size_of::<Vertex>() as i32, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, std::mem::size_of::<Vertex>() as i32, (std::mem::size_of::<u32>() * 3) as i32);
            gl.enable_vertex_attrib_array(1);*/
    
            gl.use_program(Some(program));
        }
    }
    
    pub fn draw(&mut self) {
        let gl = &self.gl;
        let width = self.width;
        let height = self.height;
    
        unsafe {
            gl.viewport(0, 0, width, height);
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
    
            for (_, mesh) in &self.meshes {
                mesh.draw(&gl);
            }
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
