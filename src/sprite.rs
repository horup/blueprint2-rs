use generational_arena::Index;

#[derive(Clone, Copy)]
pub struct Sprite {
    mesh_id:Option<Index>,
    mesh_index:i32  
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            mesh_id:None,
            mesh_index:-1
        }
    }
}
