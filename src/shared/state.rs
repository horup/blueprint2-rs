use std::collections::VecDeque;

use super::{Arena, Entity, Game};

#[derive(Clone, Default)]
pub struct State<T:Game> {
    pub entities:Arena<Entity<T>>,
    pub time:f64
}

impl<T:Game> State<T> {
    pub fn new_entity(&mut self) -> &mut Entity<T> {
        let id = self.entities.insert(Entity::default());
        let mut thing = self.entities.get_mut(&id).unwrap();
        thing
    }
}

pub struct States<T:Game> {
    pub states:VecDeque<State<T>>
}

impl<T:Game> Default for States<T> {
    fn default() -> Self {
        let mut states = VecDeque::new();
        states.push_front(State::default());
        states.push_front(State::default());
        Self {
            states
        }
    }
}

impl<T:Game> States<T> {
    pub fn current_mut(&mut self) -> &mut State<T> {
        self.states.get_mut(0).expect("No current state!")
    }

    pub fn current(&mut self) -> &State<T> {
        self.states.get(0).expect("No current state!")
    }

    pub fn previous(&self) -> &State<T> {
        self.states.get(1).expect("No previous state!")
    }

    pub fn new_state(&mut self, time:f64) -> &mut State<T> {
        let mut state = State::default();
        state.time = time;
        self.states.push_front(state);
        self.current_mut()
    }
}