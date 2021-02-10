use engine::*;
use crate::BlueprintGame;

pub struct AISystem {

}

impl System<BlueprintGame> for AISystem {
    fn update(&mut self, event:&Event<BlueprintGame>, context:&mut engine::Context<BlueprintGame>) {
        match event {
            Event::Initialize => {}
            Event::FixedStep(_, dt) => {
                for (_, transform) in context.states.current_mut().entities.query_mut::<(&mut Transform)>() {
                    transform.position.x += 1.0 * * dt as f32;
                }
            }
            Event::Draw(_, _, _) => {}
            Event::Game(_) => {}
        }
       
    }
}