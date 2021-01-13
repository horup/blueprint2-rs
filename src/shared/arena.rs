use std::todo;


#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct Index {
    pub index:u16,
    pub generation:u16
}

impl Index {
    pub fn new(index:u16) -> Self {
        Self {
            index:index,
            generation:0
        }
    }
}


#[derive(Copy, Clone)]
struct Slot<T:Copy> {
    pub index:Index,
    pub value:Option<T>
}

pub struct Arena<T:Copy> {
    vec:Vec<Slot<T>>
}

impl<T:Copy> Arena<T> {
    pub fn new() -> Self {
        Self {
            vec:Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        // TODO: can be improved by pre-calc len
        let mut len = 0;
        for slot in &self.vec {
            if slot.value.is_some() {
                len += 1;
            }
        }

        len
    }

    pub fn get_mut(&mut self, index:&Index) -> Option<&mut T> {
        if let Some(slot) = self.vec.get_mut(index.index as usize) {
            if slot.index == *index {
                if let Some(value) = &mut slot.value {
                    return Some(value);
                }
            }
        }
        None
    }

    pub fn get(&self, index:&Index) -> Option<&T> {
        if let Some(slot) = self.vec.get(index.index as usize) {
            if slot.index == *index {
                if let Some(value) = &slot.value {
                    return Some(value);
                }
            }
        }
        None
    }

    pub fn clear(&mut self) {
        self.vec.clear();
    }

    /// Frees up the `index` to be used by other values
    pub fn remove(&mut self, index:&Index) {
        if let Some(slot) = self.vec.get_mut(index.index as usize) {
            if slot.index == *index {
                slot.value = None;
            }
        }
    }

    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    /// Inserts a new value into the arena, returning the Index
    /// Finds a empty slot in the `Arena`
    /// Will allocate storage if no slot was found
    pub fn insert(&mut self, value:T) -> Index {
        let mut free:Option<Index> = None;
        for slot in self.vec.iter_mut() {
            if slot.value.is_none() {
                slot.index.generation += 1;
                free = Some(slot.index);
                break;
            }
        }

        if let None = free {
            free = Some(Index {
                generation:0,
                index:self.vec.len() as u16
            });

            self.vec.reserve(1024);
            self.vec.push(Slot {index:free.unwrap(), value:Some(value)});
        }

        free.unwrap()
    }
}

