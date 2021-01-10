use crate::shared::{Context as ContextTrait, Event, Game, States};
pub struct Context<'a, T:Game> {
    pub event:Event,
    pub states:&'a mut States<T>
}

impl<'a, T:Game> ContextTrait<T> for Context<'a, T> {
    fn event(&self) -> Event {
        self.event
    }

    fn states(&mut self) -> &mut crate::shared::States<T> {
        self.states
    }
}


