use crate::{Engine, Event, Thing};
pub struct Game {

}

impl Game {

    pub fn update(&mut self, engine:&mut Engine, event:Event) {
        match event {
            Event::Initialize => {
                engine.log("Game initialized");
                let t1 = engine.world.new_thing();
                let t2 = engine.world.new_thing();
                t2.pos.x = 0.5;
                t2.pos.y = 0.5;
            }
            Event::Update(_) => {}
            Event::Draw(_) => {}
        }
    }

    pub fn new() -> Self {
        Self {
            
        }
    }
}