use std::{collections, default, slice::{Iter, IterMut}, vec::IntoIter};


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

    pub fn invalid() -> Self {
        Self {
            index:u16::MAX,
            generation:u16::MAX
        }
    }

    pub fn is_valid(&self) -> bool {
        return *self != Self::invalid();
    }
}

impl Default for Index {
    fn default() -> Self {
        return Self::invalid()
    }
}

#[derive(Copy, Clone)]
struct Slot<T> {
    pub index:Index,
    pub value:Option<T>
}


pub trait ArenaItem {
    fn with_index(self, index:Index) -> Self;
    fn index(&self) -> Index;
}


#[derive(Clone, Default)]
pub struct Arena<T:ArenaItem> {
    vec:Vec<Slot<T>>
}

impl<T:ArenaItem> Arena<T> {
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

    pub fn resize_default(&mut self, new_size:usize) {
        let mut next_index = self.vec.len();
        self.vec.resize_with(new_size, || {
            Slot {
                index:Index {
                    index:next_index as u16,
                    generation:0
                },
                value:None
                }
            }
        );
    }

    pub fn iter(&self) -> ArenaIntoIter<T> {
        let iter = self.vec.iter();
        ArenaIntoIter {
            iter:iter
        }
    }

    pub fn iter_mut(&mut self) -> ArenaIntoIterMut<T> {
        let iter = self.vec.iter_mut();
        ArenaIntoIterMut {
            iter:iter
        }
    }

    /// Sets the `value` into the arena at the given `index` including the generation!
    /// Note: Will overwrite existing value occupinging `index.index` in the underlying arena!!!
    pub fn set(&mut self, index:&Index, value:T) {
        if (index.index as usize) < self.capacity() {
            self.resize_default(index.index as usize + 1);
        }

        let slot = self.vec.get_mut(index.index as usize).expect("slot was not returned!");
        slot.index = *index;
        slot.value = Some(value.with_index(slot.index));
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

    pub fn contains(&self, index:&Index) -> bool {
        self.get(index).is_some()
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
            self.vec.push(Slot {index:free.unwrap(), value:Some(value.with_index(free.unwrap()))});
            return free.unwrap();
        }

        free.expect("failed to insert value")
    }
}

pub struct ArenaIntoIter<'a, T> {
    iter:Iter<'a, Slot<T>>
}

pub struct ArenaIntoIterMut<'a, T> {
    iter:IterMut<'a, Slot<T>>
}

impl<'a, T> Iterator for ArenaIntoIter<'a, T> {
    type Item =  &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.iter.next() {
            if let Some(value) = &next.value {
                return Some(value);
            }
        }

        None
    }
}

impl<'a, T> Iterator for ArenaIntoIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.iter.next() {
            if let Some(value) = &mut next.value {
                return Some(value);
            }
        }

        None
    }
}