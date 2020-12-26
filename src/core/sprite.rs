use nalgebra::{Vector2, Vector3};
use super::Animation;
use super::GameWorld;


#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct SpriteID {
    pub index:u16,
    pub generation:u16
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Clip {
    Default,
    None
}

impl Default for Clip {
    fn default() -> Self {
        Clip::Default
    }
}

impl SpriteID {
    pub fn new(index:u16) -> SpriteID {
        Self {
            index:index,
            generation:0
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Locomotion {
    pub target_vel:Vector3<f32>,
    pub acceleration_max:Vector3<f32>
}

impl Default for Locomotion {
    fn default() -> Self {
        Self {
            target_vel:Vector3::default(),
            acceleration_max:Vector3::repeat(10.0) 
        }
    }
}

// TODO: find a good way to implement screen space sprites
// TODO: Add rotation
// TODO: added clipping type to control collision
// BUG: fixed draw order, maybe using z 
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Sprite<W:GameWorld> {
    id:SpriteID,
    in_use:bool,
    pub pos:Vector3<f32>,
    pub vel:Vector3<f32>,
    pub size:Vector2<f32>,
    pub visible:bool,
    pub art:W::Art,
    pub frame:f32,
    pub ext:W::Sprite,
    pub owner:u128,
    pub clip:Clip,
    pub locomotion:Locomotion,
    pub animation:Animation
}

impl<W:GameWorld> Sprite<W> {
    pub fn new(id:SpriteID, art:W::Art) -> Self {
        Self {
            id:id,
            pos:Vector3::new(0.0, 0.0, 0.0),
            vel:Vector3::new(0.0, 0.0, 0.0),
            in_use:true,
            visible:false,
            art:art,
            size:Vector2::new(1.0, 1.0),
            frame:1.0,
            ext:W::Sprite::default(),
            owner:0,
            clip:Clip::default(),
            locomotion:Locomotion::default(),
            animation:Animation::Default
        }
    }
}

impl<W:GameWorld> Sprite<W> {
    pub fn id(&self) -> &SpriteID {
        &self.id
    }

    pub fn delete(&mut self) {
        self.in_use = false;
    }

    pub fn in_use(&self) -> bool {
        self.in_use
    }
}

