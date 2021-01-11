use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct HashId(u64);

impl HashId {
    pub fn new(id:&str) -> Self {
        let mut hasher = DefaultHasher::default();
        id.hash(&mut hasher);
        let hashed = hasher.finish();
        Self(hashed)
    }
}

impl From<&str> for HashId {
    fn from(id: &str) -> Self {
        Self::new(id)
    }
}