use std::collections::HashMap;

use generational_arena::Arena;
use itertools::Itertools;
use nalgebra::Vector3;

use crate::{game::Game, shared::*, shared};

use glow::*;

use super::{SpriteMesh, Mesh};

pub struct Engine {
    pub current:World,
    pub previous:World,
    gl:glow::Context,
    pub width:i32,
    pub height:i32,
    meshes:Arena<Mesh>,
    tick_rate:u32,
    sprite_meshes:HashMap<shared::Texture, SpriteMesh>,
    initialized:bool,
    current_time:f64,
    accumulator:f64,
    t:f64
}

impl Engine {
    pub fn new(gl:glow::Context) -> Self {
        Self {
            tick_rate:20,
            current:World::new(),
            previous:World::new(),
            gl,
            width:0,
            height:0,
            meshes:Arena::new(),
            sprite_meshes:HashMap::new(),
            initialized:false,
            accumulator:0.0,
            current_time:Self::now_as_secs(),
            t:0.0
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn now_as_secs() -> f64 {
        return js_sys::Date::now() / 1000.0;
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn now_as_secs() -> f64 {
        return Instant::now().elapsed().as_secs_f64();
    }

    pub fn log(&self, s:&str) {
        log(s);
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

    pub unsafe fn draw_sprites(&mut self) {
        let textures_in_use= self.current.things.iter().map(|(_, thing)| thing.sprite.texture).unique().collect_vec();
        
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
        for (_, thing) in self.current.things.iter() {
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
    
    pub fn draw(&mut self, alpha:f64) {
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

    pub fn tick(&mut self, game:&mut Game) {
        if self.initialized == false {
            self.initialized = true;
            self.setup_shaders();
            game.on_event(self, Event::Initialize);
            self.tick(game);
            return;
        }

        let new_time = Self::now_as_secs();
        let mut frame_time = new_time - self.current_time;
        let max_time = 0.25;
        if frame_time > max_time {
            frame_time = max_time;
        }
        self.current_time = new_time;

        let frame_time = frame_time;
        self.accumulator += frame_time;
        let dt = 1.0 / self.tick_rate as f64;
        while self.accumulator >= dt {
            self.previous = self.current.clone();
            game.on_event(self, Event::Update(self.t, dt));
            self.t += dt;
            self.accumulator -= dt;
        }

        let alpha = self.accumulator / dt;
        
        game.on_event(self, Event::BeforeRender(self.current_time, frame_time, alpha));
        self.draw(alpha);
    }
}
