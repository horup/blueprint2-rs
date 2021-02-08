use std::marker::PhantomData;

use hecs::{DynamicBundle, Entity, World};

use crate::{Game, Sprite, Transform};

#[derive(Default)]
pub struct State<G:Game> {
    pub entities:World,
    pub time:f64,
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

        // clone all entities
        for (entity, _) in self.entities.iter() {
            target.insert_one(entity, ());
        }

        // clone all known components for entities
        // components not known cannot be cloned!
        // clone all standard engine components
        self.clone_components::<Transform>(&mut target);
        self.clone_components::<Sprite>(&mut target);

        // clone all game specific components.
        self.clone_components::<G::GameComponent1>(&mut target);

        Self {
            entities: target,
            time:self.time,
            marker:PhantomData::default()
        }
    }
}