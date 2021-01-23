use nalgebra::{Isometry3, Matrix, Matrix4, Point3, Vector3};

pub struct Camera {
    pub view:Isometry3<f32>,
    pub projection:Matrix4<f32>
}

impl Default for Camera {
    fn default() -> Self {

        let eye = Point3::new(0.0, 0.0, 1.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
        //let view = Isometry3::identity();
        let mut projection = Matrix4::identity();
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
