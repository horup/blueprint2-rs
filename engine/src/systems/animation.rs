use crate::{Context, Game, System};

pub struct AnimationSystem {

}

impl<G:Game> System<G> for AnimationSystem {
    fn update(&mut self, context:&mut Context<G>) {
        todo!()
    }
}