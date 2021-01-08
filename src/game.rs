use crate::{engine::*, shared};
use crate::shared::*;
pub struct Game {

}

impl Gamelike for Game {
    type GameThing = ();
    type GameEvent = ();

    fn on_event(&mut self, context:Context, event:Event) {
        match event {
            Event::Initialize => {
               /* engine.log("Game initialized");
                let t1 = engine.current.new_thing();
                let t2 = engine.current.new_thing();
                t2.pos.x = 0.5;
                t2.pos.y = 0.5;*/
            }
            Event::Update(time, dt) => {
             /*   for (_, t) in engine.current.things.iter_mut() {
                    t.pos.x += 0.1 * dt as f32;
                }*/
            }
            Event::BeforeRender(_,_,_) => {
               
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