use crate::{shared};
use crate::shared::*;
pub struct Game {

}

impl Gamelike for Game {
    type GameThing = ();
    type GameEvent = ();

    fn update(&mut self, context:Context) {
        match context.event {
            Event::Initialize => {
                log("Game initialized");
                let t1 = context.current.new_thing();
                let t2 = context.current.new_thing();
                t2.pos.x = 0.5;
                t2.pos.y = 0.5;
            }
            Event::FixedStep(time, dt) => {
                for (_, t) in context.current.things.iter_mut() {
                    t.pos.x += 0.1 * dt as f32;
                }
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