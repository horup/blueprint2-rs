use super::Context;

pub trait Game : Default {
    type GameComponent1 : Clone + Send + Sync + 'static;
    fn update(&mut self, context:&mut Context<Self>);
}