use std::{collections::HashMap, rc::Rc};

use glow::HasContext;

use crate::game::{AssetKey, Assets, Game, Sprite, SpriteMesh, SpriteSheet, States, Transform};

pub struct Renderer {
    gl:Rc<glow::Context>,
    pub width:i32,
    pub height:i32,
    sprite_meshes:HashMap<AssetKey<SpriteSheet>, SpriteMesh>
}

impl Renderer {
    pub fn new(gl:Rc<glow::Context>) -> Self {
        Self {
            gl,
            width:1,
            height:1,
            sprite_meshes:HashMap::new()
        }
    }

    
    pub fn setup_shaders(&mut self) {
        unsafe {
    
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
    
            gl.use_program(Some(program));
        }
    }

    
    pub unsafe fn draw_sprites<G:Game>(&mut self, alpha:f64, states:&States<G>, assets:&Assets) {
        let current = states.current();
        for e in current.entities.iter() {
            
        }


        for (entity, sprite) in current.entities.query::<&Sprite>().iter() {
            if self.sprite_meshes.contains_key(&sprite.spritesheet) == false {
                self.sprite_meshes.insert(sprite.spritesheet, SpriteMesh::new(&self.gl, 1024, sprite.spritesheet));
            }
        }

        for sprite_mesh in self.sprite_meshes.values_mut() {
            sprite_mesh.clear();
        }

        for (entity, (transform, sprite)) in current.entities.query::<(&Transform, &Sprite)>().iter() {
            if let Some(sprite_mesh) = self.sprite_meshes.get_mut(&sprite.spritesheet) {
                sprite_mesh.push_sprite(assets, transform, sprite);
            }
        }

        for sprite_mesh in self.sprite_meshes.values_mut() {
            sprite_mesh.update(&self.gl);
            sprite_mesh.draw(&self.gl, &assets);
        }
    }

    pub fn draw<G:Game>(&mut self, alpha:f64, states:&States<G>, assets:&Assets) {
        let width = self.width;
        let height = self.height;
    
        unsafe {
            self.gl.viewport(0, 0, width, height);
            self.gl.clear_color(0.1, 0.2, 0.3, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
            self.draw_sprites(alpha, states, assets);
          /*  for (_, mesh) in &self.meshes {
                mesh.draw(&self.gl);
            }*/
        }
    }
}