use crate::{Context, log};

pub fn update(context:Context) {
    match context.event {
        crate::Event::Initialize => {
            log("Initialize");
        }
        crate::Event::Update(_) => {
            log("Update");
        }
        crate::Event::Draw(_) => {
            log("Draw!");
        }
    }
}