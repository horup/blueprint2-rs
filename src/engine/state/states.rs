use std::collections::VecDeque;

use crate::game::Game;
use super::State;

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

    pub fn current(&self) -> &State<T> {
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