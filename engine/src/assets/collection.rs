use std::{collections::{HashMap, hash_map::{DefaultHasher, Iter, IterMut}}, default, hash::{Hash, Hasher}, marker::PhantomData, rc::Rc, todo};
use super::Assets;

pub struct AssetKey<T>(u64, PhantomData<T>);

impl<T> Clone for AssetKey<T> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<T> Default for AssetKey<T> {
    fn default() -> Self {
        Self(0, PhantomData::default())
    }
}

impl<T> Copy for AssetKey<T> {}

impl<T> Hash for AssetKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> PartialEq for AssetKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for AssetKey<T> {
    fn assert_receiver_is_total_eq(&self) {}
}


impl<T> AssetKey<T> {
    pub fn new(id:&str) -> Self {
        let mut hasher = DefaultHasher::default();
        id.hash(&mut hasher);
        let hashed = hasher.finish();
        Self(hashed, PhantomData::default())
    }
}

impl<T> From<&str> for AssetKey<T> {
    fn from(id: &str) -> Self {
        Self::new(id)
    }
}


pub struct AssetCollection<T> {
    hash_map:HashMap<AssetKey<T>, T>
}

impl<T:Default> Default for AssetCollection<T> {
    fn default() -> Self {
        let mut v = Self {
            hash_map:HashMap::new()
        };
        v.insert("default".into(), T::default());
        return v;
    }
}

impl<T:Default> AssetCollection<T> {
    pub fn insert(&mut self, key:AssetKey<T>, asset:T) {
        self.hash_map.insert(key, asset);
    }

    pub fn iter(&self) -> Iter<'_, AssetKey<T>, T> {
        self.hash_map.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, AssetKey<T>, T> {
        self.hash_map.iter_mut()
    }

    pub fn get(&self, key:&AssetKey<T>) -> &T {
        if self.hash_map.contains_key(key) {
            return self.hash_map.get(key).unwrap();
        }
        else {
            return self.hash_map.get(&"default".into()).unwrap();
        }
    }

    pub fn get_mut(&mut self, key:&AssetKey<T>) -> &mut T {

        if self.hash_map.contains_key(key) {
            return self.hash_map.get_mut(key).unwrap();
        }
        else {
            return self.hash_map.get_mut(&"default".into()).unwrap();
        }
    }
}

