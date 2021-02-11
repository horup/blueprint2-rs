use engine::{Context, Event, Game, Sprite, System, log};

#[derive(Default)]
pub struct Animator {

}

impl<G:Game> System<G> for Animator {
    fn on_event(&mut self, event:&Event<G>, context:&mut Context<G>) {
        match event {
            engine::Event::Initialize => {}
            engine::Event::Step(_, dt) => {
                let current = context.states.current_mut();
                for (_, sprite) in current.entities.query_mut::<(&mut Sprite)>() {
                    sprite.frame += 1.0 * *dt as f32;
                }
            }
            engine::Event::Draw(_, _, _) => {}
            engine::Event::GameEvent(_) => {}
        }
    }
}