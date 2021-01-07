use std::collections::HashMap;

use generational_arena::Arena;
use itertools::Itertools;
use nalgebra::Vector3;
use crate::{Event, Game, Mesh, SpriteMesh, Timer, World, log};
use glow::*;

pub struct Engine {
    pub world:World,
    pub gl:glow::Context,
    pub width:i32,
    pub height:i32,
    pub meshes:Arena<Mesh>,
    timer:Timer,
    sprite_meshes:HashMap<crate::Texture, SpriteMesh>
}

impl Engine {
    pub fn new(gl:glow::Context) -> Self {
        Self {
            world:World::new(),
            gl,
            width:0,
            height:0,
            meshes:Arena::new(),
            timer:Timer::default(),
            sprite_meshes:HashMap::new()
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

    pub unsafe fn draw_sprites(&mut self) {
        let textures_in_use= self.world.things.iter().map(|(_, thing)| thing.sprite.texture).unique().collect_vec();
        
        // ensure a sprite_mesh exist for all textures in use
        for texture in &textures_in_use {
            if self.sprite_meshes.contains_key(texture) == false {
                self.sprite_meshes.insert(*texture, SpriteMesh::new(&self.gl, 1024));
            }
        }

        // clear all sprite meshes to 0 sprites
        for sprite_mesh in self.sprite_meshes.values_mut() {
            sprite_mesh.clear();
        }

        // populate the sprite meshes with sprite data
        for (_, thing) in self.world.things.iter() {
            if let Some(sprite_mesh) = self.sprite_meshes.get_mut(&thing.sprite.texture) {
                sprite_mesh.push_sprite(thing.pos);
            }
        }

        // draw the sprite meshes one by one
        for sprite_mesh in self.sprite_meshes.values_mut() {
            sprite_mesh.update(&self.gl);
            sprite_mesh.draw(&self.gl);
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

    pub fn on_event(&mut self, e:Event, game:&mut Game) {
        match e {
            Event::Initialize => {
                self.setup_shaders();
                game.on_event(self, e);
            }
            Event::Update(_) => {
                game.on_event(self, e);
            }
            Event::Draw(_) => {
                game.on_event(self, e);
                self.draw();
                self.timer.on_draw();
                log(&format!("{}", self.timer.draw_delta_avg()));
            }
        }
    }
}
