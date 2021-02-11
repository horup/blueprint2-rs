use engine::*;
use crate::BlueprintGame;

pub struct AISystem {

}

impl System<BlueprintGame> for AISystem {
    fn on_event(&mut self, event:&Event<BlueprintGame>, context:&mut engine::Context<BlueprintGame>) {
        match event {
            Event::Initialize => {}
            Event::Step(_, dt) => {
                for (_, transform) in context.states.current_mut().entities.query_mut::<(&mut Transform)>() {
                    transform.position.x += 1.0 * * dt as f32;
                }
            }
            Event::Draw(_, _, _) => {}
            Event::GameEvent(_) => {}
        }
       
    }
}