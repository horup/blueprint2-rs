
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
    pub in_use:bool,
    pub value:T
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

    pub fn get_mut(&mut self, index:&Index) -> Option<T> {
       None
    }

    pub fn insert(&mut self, value:T) -> Index {
        
        let mut free:Option<Index> = None;
        for slot in self.vec.iter_mut() {
            if !slot.in_use {
                slot.index.generation += 1;
                free = Some(slot.index);
            }
        }

        if let None = free {
            free = Some(Index {
                generation:0,
                index:self.vec.len() as u16
            });

            self.vec.push(Slot {in_use:true, index:free.unwrap(), value:value});
        }

        free.unwrap()
    }
}

