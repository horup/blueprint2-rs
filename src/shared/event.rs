pub enum Event {
    Initialize,
    /// `time`, `delta_time`
    Update(f64, f64),

    /// `time`, `delta_time`, `alpha`
    BeforeRender(f64, f64, f64)
}