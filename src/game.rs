use image::DynamicImage;
use nalgebra::Vector3;

pub use crate::engine::*;
use crate::shared::log;

#[derive(Default)]
pub struct BlueprintGame {

}

impl Game for BlueprintGame {
    type GameComponent1 = ();

    fn update(&mut self, context:&mut Context<Self>) {
        let current = context.states.current_mut();
        match context.event {
            Event::Initialize => {
                log("Game initialized");
                let mut assets = &mut context.assets;
                let sheet01 = assets.load_texture_from_png_bytes("sheet01".into(), include_bytes!("./assets/textures/spritesheet.png"));
                
                let frames = [sheet01.frame(0, 0, 16, 16)];
                assets.load_spritesheet("sheet01".into(), 
                SpriteSheet {
                    texture:"sheet01".into(),
                    frames:frames.into()
                });

                current.entities.spawn((
                    Transform { position:Vector3::new(0.0, 0.0, 0.0)},
                    Sprite::default()
                ));

                current.entities.spawn((
                    Transform { position:Vector3::new(0.5, 0.5, 0.0)},
                    Sprite::default()
                ));

               
            }
            Event::FixedStep(time, dt) => {
                for (_, t) in current.entities.query_mut::<&mut Transform>() {
                    t.position.x += 0.1 * dt as f32;
                }
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