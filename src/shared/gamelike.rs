use super::{Context, Event};
pub trait Gamelike {
    type GameEntity;
    type GameEvent;
    type Texture;
    type Spritesheet;

    fn update(&mut self, context:Context);
}