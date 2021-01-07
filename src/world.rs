use generational_arena::{Arena, IterMut};

use crate::Thing;

#[derive(Clone)]
pub struct World {
    pub things:Arena<Thing>
}

impl World {
    pub fn new() -> Self {
        Self {
            things:Arena::new()
        }
    }

    pub fn new_thing(&mut self) -> &mut Thing {
        let id = self.things.insert(Thing::default());
        let mut thing = self.things.get_mut(id).unwrap();
        thing
    }
}