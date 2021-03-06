use std::{collections::HashMap, rc::Rc};

use itertools::Itertools;
use nalgebra::Vector3;
use winit::window::Window;

use crate::Systems;

use super::{AssetKey, Assets, Event, Game, SpriteSheet, States, System, Transform, log, Context as SharedContext};
use super::{engine_systems::Renderer};

pub struct Engine<G:Game> {
    pub assets:Assets,
    pub states:States<G>,
    gl:Rc<glow::Context>,
    tick_rate:u32,
    initialized:bool,
    current_time:f64,
    accumulator:f64,
    t:f64,
    pub renderer:Renderer,
    pub systems:Systems<G>
}

impl<T:Game + 'static> Engine<T> {
    pub fn new(gl:glow::Context) -> Self {
        let gl = Rc::new(gl);
        Self {
            tick_rate:20,
            states:States::default(),
            gl:gl.clone(),
            initialized:false,
            accumulator:0.0,
            current_time:Self::now_as_secs(),
            t:0.0,
            assets:Assets::new(gl.clone()),
            renderer:Renderer::new(gl.clone()),
            systems:Systems::default()
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
    

    fn emit_event(&mut self, event:&Event<T>) {
        let mut c = SharedContext {
           /* current:&mut self.current,
            previous:&mut self.previous,*/
            states:&mut self.states,
            assets:&mut self.assets,
            camera:&mut self.renderer.camera
        };

        self.systems.on_event(&event, &mut c);
        //self.game.update(&mut c);
    }

    pub fn update(&mut self, window:&mut Window, game:&mut T) {
        if self.initialized == false {
            self.initialized = true;
            self.renderer.setup_shaders();
            self.emit_event(&Event::Initialize);
            game.setup(self);
            self.assets.update();
            self.update(window, game);

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
            //self.previous = self.current.clone();
            let t = self.t;
            self.emit_event(&Event::Step(t as f32, dt as f32));
            self.t += dt;
            self.accumulator -= dt;
        }

        let alpha = self.accumulator / dt;
        
        let current_time = self.current_time;
        //self.game.update(self.create_context(Event::Draw(current_time, frame_time, alpha)));
        self.renderer.draw(alpha, &self.states, &self.assets, window);
    }
}
