use nalgebra::Vector3;

/// A Component which transform an entity in world space.
/// Nearly every entity has a transform
#[derive(Clone, Copy, Debug, Default)]
pub struct Transform {
    pub position:Vector3<f32>
}