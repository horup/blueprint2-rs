use engine::*;
use crate::BlueprintGame;

pub struct AISystem {

}

impl System<BlueprintGame> for AISystem {
    fn on_step(&mut self, time:f32, delta_time:f32, context:&mut Context<BlueprintGame>) {
        for (_, transform) in context.states.current_mut().entities.query_mut::<(&mut Transform)>() {
            transform.position.x += 1.0 * delta_time as f32;
        }
    }
}