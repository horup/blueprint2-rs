use generational_arena::{Arena, IterMut};

use super::Entity;

#[derive(Clone)]
pub struct State {
    pub entities:Arena<Entity>
}

impl State {
    pub fn new() -> Self {
        Self {
            entities:Arena::new()
        }
    }

    pub fn new_entity(&mut self) -> &mut Entity {
        let id = self.entities.insert(Entity::default());
        let mut thing = self.entities.get_mut(id).unwrap();
        thing
    }
}