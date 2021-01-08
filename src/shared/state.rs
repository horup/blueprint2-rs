use generational_arena::{Arena, IterMut};

use super::{Entity, Gamelike};

#[derive(Clone)]
pub struct State<T:Gamelike> {
    pub entities:Arena<Entity<T>>
}

impl<T:Gamelike> State<T> {
    pub fn new() -> Self {
        Self {
            entities:Arena::new()
        }
    }

    pub fn new_entity(&mut self) -> &mut Entity<T> {
        let id = self.entities.insert(Entity::default());
        let mut thing = self.entities.get_mut(id).unwrap();
        thing
    }
}