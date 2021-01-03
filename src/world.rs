use generational_arena::Arena;

use crate::Thing;

pub struct World {
    pub things:Arena<Thing>
}


impl World {
    pub fn new() -> Self {
        Self {
            things:Arena::new()
        }
    }
}