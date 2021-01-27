use core::num;

use image::DynamicImage;
use nalgebra::Vector3;

pub use crate::engine::*;
use crate::{engine, shared::log};

#[derive(Default)]
pub struct BlueprintGame {

}

impl Game for BlueprintGame {
    type GameEvent = ();
    type GameComponent1 = ();

    fn update(&mut self, context:&mut Context<Self>) {
        let current = context.states.current_mut();
        match context.event {
            Event::Initialize => {
                log("Game initialized");
                let mut assets = &mut context.assets;
                let sheet01 = assets.load_texture_from_png_bytes("sheet01".into(), include_bytes!("./assets/textures/spritesheet.png"));
                
                let frames = [
                    sheet01.frame(0, 0, 16, 16), 
                    sheet01.frame(0, 16, 16, 16)];
                assets.load_spritesheet("sheet01".into(), 
                SpriteSheet {
                    texture:"sheet01".into(),
                    frames:frames.into()
                });

                context.camera.zoom = 20.0;

                let max = 10;
                for i in 0..max*max {
                    let x = i % max;
                    let y = i / max;

                    current.entities.spawn((

                        Transform { position:Vector3::new(x as f32 , y as f32, 0.0)},
                        Sprite {
                            frame:0,
                            spritesheet:"sheet01".into()
                        }
                    ));
    
                }
            }
            Event::FixedStep(time, dt) => {
                for (_, t) in current.entities.query_mut::<&mut Transform>() {
                    //t.position.x += 0.1 * dt as f32;
                }

                for (_, s) in current.entities.query_mut::<&mut Sprite>() {
                    //t.position.x += 0.1 * dt as f32;
                    s.frame += 1;
                }
            }
            Event::Draw(_,_,_) => {
               
            }
            Event::Game(_) => {}
        }
    }

}

impl BlueprintGame {
    pub fn new() -> Self {
        Self {
            
        }
    }
}