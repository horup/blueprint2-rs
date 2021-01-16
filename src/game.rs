use image::DynamicImage;

pub use crate::engine::*;
use crate::shared::log;

#[derive(Default)]
pub struct BlueprintGame {

}

impl Game for BlueprintGame {
    type GameComponent1 = ();

    fn update(&mut self, context:&mut Context<Self>) {
        match context.event {
            Event::Initialize => {
                log("Game initialized");
                let state = context.states.current_mut();
                let mut assets = &mut context.assets;
                let sheet01 = assets.load_texture_from_png_bytes("sheet01".into(), include_bytes!("./assets/textures/spritesheet.png"));
                
                let frames = [sheet01.frame(0, 0, 16, 16)];
                assets.load_spritesheet("sheet01".into(), 
                SpriteSheet {
                    texture:"sheet01".into(),
                    frames:frames.into()
                });
                
                /*let t1 = state.new_entity();
                let t2 = state.new_entity();
                t2.pos.x = 0.5;
                t2.pos.y = 0.5;*/
            }
            Event::FixedStep(time, dt) => {
               /* for (_, t) in context.current.entities.iter_mut() {
                    t.pos.x += 0.1 * dt as f32;
                }*/
            }
            Event::Draw(_,_,_) => {
               
            }
        }
    }

}

impl BlueprintGame {
    pub fn new() -> Self {
        Self {
            
        }
    }
}