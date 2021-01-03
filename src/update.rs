use crate::{Engine, Event, draw, log, setup_shaders};

pub fn update(context:&mut Engine, e:Event) {
    match e {
        Event::Initialize => {
            setup_shaders(context);
        }
        Event::Update(_) => {}
        Event::Draw(_) => {
            draw(context);
        }
    }
  /*  match context.event {
        crate::Event::Initialize => {
            log("Initialize");
        }
        crate::Event::Update(_) => {
            log("Update");
        }
        crate::Event::Draw(_) => {
            log("Draw!");
        }
    }*/
}