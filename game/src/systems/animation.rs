use engine::{Context, Event, Game, Sprite, System, log};

#[derive(Default)]
pub struct Animator {

}

impl<G:Game> System<G> for Animator {
    fn on_step(&mut self, time:&f32, delta_time:&f32, context:&mut Context<G>) {
        let current = context.states.current_mut();
        for (_, sprite) in current.entities.query_mut::<(&mut Sprite)>() {
            sprite.frame += 1.0 * * delta_time as f32;
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}