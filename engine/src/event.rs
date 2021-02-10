use super::Game;


#[derive(Copy, Clone)]
pub enum Event<G:Game> {
    Initialize,
    /// `time`, `delta_time`
    FixedStep(f32, f32),

    /// `time`, `delta_time`, `alpha`
    Draw(f32, f32, f32),

    Game(G::GameEvent)
}