use nalgebra::{Isometry, Isometry3, Matrix, Matrix4, Orthographic3, Perspective3, Point3, Vector3};

pub struct Camera {
    pub view:Isometry3<f32>,
    pub projection:Matrix4<f32>,
    pub zoom:f32
}

impl Default for Camera {
    fn default() -> Self {
        let eye = Point3::new(0.0, 0.0, 1.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let mut me = Self {
            view:Isometry3::identity(),
            projection:Matrix4::identity(),
            zoom:1.0,
        };

    //    me.lookat(eye, target, up);
    //    me.set_orthogonal_projection(1.0, 1.0);
        return me;
    }
}

impl Camera {

    pub fn update(&mut self, width:f32, height:f32) {
        let aspect = width / height;
        let zoom = self.zoom;
        let near = 1.0;
        let far = -1.0;

        if aspect > 1.0 {
            let w = zoom * aspect;
            let h = zoom;
            let orth = Orthographic3::new(-w/2.0, w/2.0, -h/2.0, h/2.0, near, far);
            self.projection = orth.to_homogeneous();
        }
        else {
            let w = zoom;
            let h = zoom / aspect;
            let orth = Orthographic3::new(-w/2.0, w/2.0, -h/2.0, h/2.0, near, far);
            self.projection = orth.to_homogeneous();
        }

        //let view = Isometry3::look_at_rh(&self.eye, &self.target, &self.up);
        //self.view = view;
        //let znear = -1.0;
        //let zfar = 1.0;
        //let mut projection = Orthographic3::new(-width/2.0, width/2.0, -height/2.0, height/2.0, znear, zfar).to_homogeneous();
        //self.projection = projection;
        //let mut projection = Orthographic3::new(-self.width/2.0, self.width/2.0, -self.height/2.0, self.height/2.0, znear, zfar).to_homogeneous();
        //self.projection = projection;
    }

    pub fn view_projection(&self) -> Matrix4<f32> {
        self.projection * self.view.to_homogeneous()
    }

    pub fn lookat(&mut self, eye:Point3<f32>, target:Point3<f32>, up:Vector3<f32>) {
      //  let view = Isometry3::look_at_rh(&eye, &target, &up);
       // self.view = view;
    }
}
