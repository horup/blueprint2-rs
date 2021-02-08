use engine::{Context, Game, System};

pub struct Animator {

}

impl<G:Game> System<G> for Animator {
    fn update(&mut self, context:&mut Context<G>) {
    }
}