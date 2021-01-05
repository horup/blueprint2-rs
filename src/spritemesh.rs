use glow::Context;
use crate::Mesh;
use nalgebra::Vector3;


/// An object which maintains a single mesh consisting of one or more sprites
pub struct SpriteMesh {

    /// The mesh which holds the raw vertex data
    pub mesh:Mesh,
    /// Max number of sprites, sprites exceeding this number will not be drawn
    pub max_sprites:usize,
    /// Number of sprites to draw
    pub count:usize
}

impl SpriteMesh {

    /// Instantiate a new SpriteMesh object with `max_sprites` being the maximum number of quads 
    /// in the mesh.
    pub unsafe fn new(gl:&mut Context, max_sprites:usize) -> Self {
        
        let mesh = Mesh::new_quads(gl, max_sprites);

        Self {
            max_sprites:max_sprites,
            mesh:mesh,
            count:0
        }
    }

    pub unsafe fn draw(&self, gl:&mut Context) {

    }

    pub fn clear(&mut self) {
        self.count = 0;
    }

    /// Pushes a sprite to the mesh, which is drawn by calling `draw`
    pub fn push_sprite(&mut self, pos:Vector3<f32>) {
        /*
          for i in 0..count {
            // lower right triangle
            vertices.push(Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0)); //1
            vertices.push(Vertex::new(0.5, -0.5, 0.0, 1.0, 0.0)); //2
            vertices.push(Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0)); //3

            // upper left triangle
            vertices.push(Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0)); //1
            vertices.push(Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0)); //3
            vertices.push(Vertex::new(-0.5, 0.5, 0.0, 0.0, 1.0)); //4
        }
        */

        if self.count < self.max_sprites {
            let mesh = &mut self.mesh;
            let i = self.count * 6;

            self.count += 1;
        }
    }
}