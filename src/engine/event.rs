use super::Game;


#[derive(Copy, Clone)]
pub enum Event<G:Game> {
    Initialize,
    /// `time`, `delta_time`
    FixedStep(f64, f64),

    /// `time`, `delta_time`, `alpha`
    Draw(f64, f64, f64),

    Game(G::GameEvent)
}