use nalgebra::{Isometry3, Matrix, Matrix4, Orthographic3, Perspective3, Point3, Vector3};

pub struct Camera {
    pub view:Isometry3<f32>,
    pub projection:Matrix4<f32>
}

impl Default for Camera {
    fn default() -> Self {
        let eye = Point3::new(0.0, 0.0, 1.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
        let s = 4.0;
        let mut projection = Orthographic3::new(-s, s, -s, s, 1.0, -100.0).to_homogeneous(); //Matrix4::identity();
        projection = projection * 0.1;
        Self {
            view,
            projection
        }
    }
}

impl Camera {
    pub fn view_projection(&self) -> Matrix4<f32> {
        self.projection * self.view.to_homogeneous()
    }
}
