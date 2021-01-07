use std::time::Instant;

#[derive(Default)]
pub struct Timer {
    start_time:f64,
    last_draw_time:f64,
    draw_delta:f64
}

impl Timer {

    pub fn on_draw(&mut self) {
        let now = Self::now_as_secs_f64();
        let elapsed = now - self.last_draw_time;
        self.last_draw_time = now;
        self.draw_delta = elapsed;
    }

    pub fn draw_delta_avg(&self) -> f64 {
        self.draw_delta
    }

    pub fn elapsed_as_secs(&self) -> f64 {
        return Self::now_as_secs_f64() - self.start_time;
    }

    #[cfg(target_arch = "wasm32")]
    pub fn now_as_secs_f64() -> f64 {
        return js_sys::Date::now() / 1000.0;
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn now_as_secs_f64() -> f64 {
        return Instant::now().elapsed().as_secs_f64();
    }
}