use std::time;
use std::time::Instant;

pub struct Time {
    start: time::Instant,
    now: time::Instant,
}
impl ecs::Global for Time {}

impl Time {
    pub fn instant(&self) -> f32  {
        let instant_ns = self.now.duration_since(self.start).as_nanos() as u64;
        let instant = ((instant_ns/1000) as f32) / 1_000_000.0;
        instant
    }
    pub fn update(&mut self) {
        self.now = time::Instant::now();
    }
    pub fn new() -> Self {
        let now = time::Instant::now();
        Self {
            start: now,
            now: now,
        }
    }
}
