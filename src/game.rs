use crate::{Engine, Event, Thing, log};
pub struct Game {

}

impl Game {

    pub fn on_event(&mut self, engine:&mut Engine, event:Event) {
        match event {
            Event::Initialize => {
                engine.log("Game initialized");
                let t1 = engine.world.new_thing();
                let t2 = engine.world.new_thing();
                t2.pos.x = 0.5;
                t2.pos.y = 0.5;
            }
            Event::Update(_) => {
                for (_, t) in engine.world.things.iter_mut() {
                    t.pos.x += 0.01;
                }
            }
            Event::Draw(_) => {
               
            }
        }
    }

    pub fn new() -> Self {
        Self {
            
        }
    }
}