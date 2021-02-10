use engine::{Context, Event, Game, Sprite, System, log};

pub struct Animator {

}

impl<G:Game> System<G> for Animator {
    fn update(&mut self, event:&Event<G>, context:&mut Context<G>) {
        match event {
            engine::Event::Initialize => {}
            engine::Event::FixedStep(_, dt) => {
                let current = context.states.current_mut();
                for (_, sprite) in current.entities.query_mut::<(&mut Sprite)>() {
                    sprite.frame += 1.0 * *dt as f32;
                }
            }
            engine::Event::Draw(_, _, _) => {}
            engine::Event::Game(_) => {}
        }
    }
}