use std::marker::PhantomData;

use hecs::{DynamicBundle, Entity, World};

use super::Game;

#[derive(Default)]
pub struct State<G:Game> {
    pub entities:World,
    marker:PhantomData<G>
}

impl<G:Game> State<G> {
    
    fn clone_components<T : Clone + Send + Sync + 'static>(&self, target:&mut World) {
        for (entity, component) in self.entities.query::<&T>().iter() {
            target.insert_one(entity, component.clone());
        }
    }
}


impl<G:Game> Clone for State<G> {
    fn clone(&self) -> Self {
        let mut target = World::new();
        for (entity, _) in self.entities.iter() {
            target.insert_one(entity, ());
        }

        self.clone_components::<G::GameComponent1>(&mut target);

        Self {
            entities: target,
            marker:PhantomData::default()
        }
    }
}