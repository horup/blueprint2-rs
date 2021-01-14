
#[derive(Copy, Clone)]
pub enum Event {
    Initialize,
    /// `time`, `delta_time`
    FixedStep(f64, f64),

    /// `time`, `delta_time`, `alpha`
    Draw(f64, f64, f64)
}