use std::{collections::HashMap, rc::Rc};

use glow::{HasContext, ONE_MINUS_SRC_ALPHA, SRC_ALPHA, WebProgramKey};
use nalgebra::{Matrix, Matrix4};
use winit::window::Window;

use crate::{AssetKey, SpriteMesh, SpriteSheet, Assets, Game, Sprite, States, Transform};

use super::Camera;

pub struct Renderer {
    gl:Rc<glow::Context>,
    pub width:i32,
    pub height:i32,
    sprite_meshes:HashMap<AssetKey<SpriteSheet>, SpriteMesh>,
    pub camera:Camera,
    pub program:Option<WebProgramKey>
}

impl Renderer {
    pub fn new(gl:Rc<glow::Context>) -> Self {
        Self {
            gl,
            width:1,
            height:1,
            sprite_meshes:HashMap::new(),
            camera:Camera::default(),
            program:None
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
    
            self.program = Some(program);
            gl.use_program(self.program);
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

        if let Some(program) = self.program {
            for sprite_mesh in self.sprite_meshes.values_mut() {
                sprite_mesh.update(&self.gl);
                sprite_mesh.draw(&self.gl, program, &assets, &self.camera);
            }
        }
    }

    pub fn draw<G:Game>(&mut self, alpha:f64, states:&States<G>, assets:&Assets, window:&mut Window) {
        let width = self.width;
        let height = self.height;
        let inner = window.inner_size();
        unsafe {
            self.gl.viewport(0, 0, inner.width as i32, inner.height as i32);
            //self.camera.set_orthogonal_projection(inner.width as f32 / 100.0, inner.height as f32 / 100.0);
            self.camera.update(inner.width as f32, inner.height as f32);

            self.gl.clear_color(0.1, 0.2, 0.3, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
            self.gl.enable(glow::BLEND);
            self.gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            self.draw_sprites(alpha, states, assets);
          /*  for (_, mesh) in &self.meshes {
                mesh.draw(&self.gl);
            }*/
        }
    }
}