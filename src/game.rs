pub mod Game {
    use crate::{Engine, Event, Thing};

    pub fn update(engine:&mut Engine, event:Event) {
        match event {
            Event::Initialize => {
                engine.log("Game initialized");
                let t1 = engine.world.new_thing();
            }
            Event::Update(_) => {}
            Event::Draw(_) => {}
        }
    }
}