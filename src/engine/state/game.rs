pub trait Game {
    type GameComponent1 : Clone + Send + Sync + 'static;
}