use crate::{Engine, Context};
pub trait Game : Default {
    type GameEvent;
    type GameComponent1 : Clone + Send + Sync + 'static;
    fn setup(&mut self, engine:&mut Engine<Self>);
    //fn update(&mut self, context:&mut Context<Self>);
}