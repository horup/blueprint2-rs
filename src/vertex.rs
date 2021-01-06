
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pub x:f32,
    pub y:f32,
    pub z:f32,
    pub u:f32,
    pub v:f32
}

impl Vertex {
    pub fn new(x:f32, y:f32, z:f32, u:f32, v:f32) -> Self {
        Self {x, y, z, u, v}
    }

    pub fn set(&mut self, x:f32, y:f32, z:f32, u:f32, v:f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.u = u;
        self.v = v;
    }
}