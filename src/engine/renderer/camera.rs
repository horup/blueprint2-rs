use nalgebra::{Isometry, Isometry3, Matrix, Matrix4, Orthographic3, Perspective3, Point3, Vector3};

pub struct Camera {
    pub view:Isometry3<f32>,
    pub projection:Matrix4<f32>
}

impl Default for Camera {
    fn default() -> Self {
        let eye = Point3::new(0.0, 0.0, 1.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
        let up = Vector3::y();
        let s = 4.0;
        let mut me = Self {
            view: Isometry::identity(),
            projection: Matrix4::identity()
        };

        me.lookat(eye, target, up);
        me.set_orthogonal_projection(1.0, 1.0);
        return me;
    }
}

impl Camera {
    pub fn view_projection(&self) -> Matrix4<f32> {
        self.projection * self.view.to_homogeneous()
    }

    pub fn set_orthogonal_projection(&mut self, width:f32, height:f32) {
        let znear = 1.0;
        let zfar = -100.0;
        let mut projection = Orthographic3::new(-width/2.0, width/2.0, -height/2.0, height/2.0, znear, zfar).to_homogeneous();
        self.projection = projection;
    }

    pub fn lookat(&mut self, eye:Point3<f32>, target:Point3<f32>, up:Vector3<f32>) {
        let view = Isometry3::look_at_rh(&eye, &target, &up);
        self.view = view;
    }
}
