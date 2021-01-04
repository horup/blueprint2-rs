use crate::{Engine, Event, Thing};
pub struct Game {

}

impl Game {

    pub fn update(&mut self, engine:&mut Engine, event:Event) {
        match event {
            Event::Initialize => {
                engine.log("Game initialized");
                let t1 = engine.world.new_thing();
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