use generational_arena::Arena;
use crate::{Event, Game, Mesh, World, log};
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

    pub fn setup_sprites(&mut self) {
        unsafe {
            let mesh = Mesh::new_quads(&mut self.gl, 1024);
            self.meshes.insert(mesh);
        }
    }

    pub fn log(&self, s:&str) {
        log(s);
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
    
            gl.use_program(Some(program));
        }
    }

    pub fn draw_sprites(&mut self) {
        for (_, thing) in self.world.things.iter_mut() {
            log("test"); 
        }
    }
    
    pub fn draw(&mut self) {
        let width = self.width;
        let height = self.height;

        
    
        unsafe {
            self.gl.viewport(0, 0, width, height);
            self.gl.clear_color(0.1, 0.2, 0.3, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
            self.draw_sprites();
            for (_, mesh) in &self.meshes {
                mesh.draw(&self.gl);
            }
        }
    }

    pub fn update(&mut self, e:Event, game:&mut Game) {
        match e {
            Event::Initialize => {
                self.setup_shaders();
                self.setup_sprites();
                game.update(self, e);
            }
            Event::Update(_) => {}
            Event::Draw(_) => {
                self.draw();
            }
        }
    }
}
