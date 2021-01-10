use crate::{shared::{Context, Event, Game as GameTrait, HashId, log}};
#[derive(Default)]
pub struct Game {

}

impl GameTrait for Game {
    type GameEntity =  ();
    type GameEvent = ();

    fn update(&mut self, context:&mut Context<Self>) {
        match context.event() {
            Event::Initialize => {
                log("Game initialized");

                let bytes = include_bytes!("./assets/textures/spritesheet.png");
                let img = image::load_from_memory(bytes).unwrap();
                context.assets_mut().load_texture(HashId::new("spritesheet01"), img);
             /*    let t1 = context.current.new_entity();
                let t2 = context.current.new_entity();
                t2.pos.x = 0.5;
                t2.pos.y = 0.5;*/

                
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