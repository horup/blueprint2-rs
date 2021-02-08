use glow::{Context, HasContext, WebProgramKey};
use nalgebra::Vector3;

use super::{Mesh, Vertex};
use crate::{Assets, Camera, Sprite, Transform, engine::{AssetKey, SpriteSheet}};


/// An object which maintains a single mesh consisting of one or more sprites
pub struct SpriteMesh {
    /// The mesh which holds the raw vertex data
    mesh:Mesh,
    /// Max number of sprites, sprites exceeding this number will not be drawn
    pub max_sprites:usize,
    /// Number of sprites to draw
    pub count:usize,
    /// The spritesheet to use
    pub sprite_sheet:AssetKey<SpriteSheet>
}

impl SpriteMesh {

    /// Instantiate a new SpriteMesh object with `max_sprites` being the maximum number of quads 
    /// in the mesh.
    pub unsafe fn new(gl:&Context, max_sprites:usize, sprite_sheet:AssetKey<SpriteSheet>) -> Self {
        
        let mesh = Mesh::new_quads(gl, max_sprites);

        Self {
            max_sprites:max_sprites,
            mesh:mesh,
            count:0,
            sprite_sheet:sprite_sheet
        }
    }

    pub unsafe fn draw(&self, gl:&Context, program:WebProgramKey, assets:&Assets, camera:&Camera) {
        let sheet = assets.spritesheets.get(&self.sprite_sheet);
        let texture = assets.textures.get(&sheet.texture);
        gl.bind_texture(glow::TEXTURE_2D, Some(texture.texture));
        self.mesh.draw_subset(gl, program, self.count * 6, camera);
    }

    pub fn clear(&mut self) {
        self.count = 0;
    }

    /// Pushes a sprite to the mesh, which is drawn by calling `draw`
    /// does not allocate memory. 
    /// sprites above max_sprites are ignored and will not be drawn
    pub fn push_sprite(&mut self, assets:&Assets, transform:&Transform, sprite:&Sprite) {
        if self.count < self.max_sprites {
            let mesh = &mut self.mesh;
            let i = self.count * 6;

            let sprite_sheet = assets.spritesheets.get(&sprite.spritesheet);
            let frame = sprite_sheet.get_frame(sprite.frame as usize);
            let mut vs = [
                Vertex::new(-0.5, -0.5, 0.0, frame.u, frame.v),
                Vertex::new(0.5, -0.5, 0.0, frame.u + frame.w, frame.v),
                Vertex::new(0.5, 0.5, 0.0, frame.u + frame.w, frame.v + frame.h),

                Vertex::new(-0.5, -0.5, 0.0, frame.u, frame.v),
                Vertex::new(0.5, 0.5, 0.0, frame.u + frame.w, frame.v + frame.h),
                Vertex::new(-0.5, 0.5, 0.0, frame.u, frame.v + frame.h)];

            // translate
            for v in &mut vs {
                v.x += transform.position.x;
                v.y += transform.position.y;
                v.z += transform.position.z;
            }

            mesh.copy_from(&vs, self.count * 6);
            self.count += 1;
        }
    }

    /// Updates the SpriteMesh with the contained vertex data.
    pub unsafe fn update(&mut self, gl:&Context) {
        self.mesh.update(gl);
    }
}