use image::DynamicImage;

use crate::{shared::{Context, Event, Game as GameTrait, HashId, log}};
#[derive(Default)]
pub struct Game {

}

impl GameTrait for Game {
    type GameEntity =  ();
    type GameEvent = ();

    fn update(&mut self, context:&mut Context<Self>) {
        match context.event {
            Event::Initialize => {
                log("Game initialized");
                let state = context.states.current_mut();
                let mut assets = &mut context.assets;
                assets.load_texture_from_bytes("spritesheet01", include_bytes!("./assets/textures/spritesheet.png"));
                
                let t1 = state.new_entity();
                let t2 = state.new_entity();
                t2.pos.x = 0.5;
                t2.pos.y = 0.5;
            }
            Event::FixedStep(time, dt) => {
               /* for (_, t) in context.current.entities.iter_mut() {
                    t.pos.x += 0.1 * dt as f32;
                }*/
            }
            Event::Draw(_,_,_) => {
               
            }
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            
        }
    }
}