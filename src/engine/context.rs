use crate::shared::{Context as ContextTrait, Assets as AssetsTrait, Event, Game, States};
use super::Assets;
pub struct Context<'a, T:Game> {
    pub event:Event,
    pub states:&'a mut States<T>,
    pub assets:&'a mut Assets
}

impl<'a, T:Game> ContextTrait<T> for Context<'a, T> {
    fn event(&self) -> Event {
        self.event
    }

    fn states(&mut self) -> &mut crate::shared::States<T> {
        self.states
    }

    fn assets_mut(&mut self) -> &mut dyn AssetsTrait {
        self.assets
    }
}


