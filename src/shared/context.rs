use super::World;

pub struct Context<'a> {
    pub current:&'a mut World
}