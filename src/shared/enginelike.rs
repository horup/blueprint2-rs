use super::Event;

pub trait Enginelike {
    fn event(&self) -> Event;
}