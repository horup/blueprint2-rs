use crate::{Engine, Event};

#[derive(Default)]
pub struct Game {

}

impl Game {
    pub fn update(engine:&mut Engine, event:Event) {
        match event {
            Event::Initialize => {}
            Event::Update(_) => {}
            Event::Draw(_) => {}
        }
    }
}