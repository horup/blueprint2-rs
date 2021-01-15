pub trait Game : Default {
    type GameComponent1 : Clone + Send + Sync + 'static;
}