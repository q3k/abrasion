use std::time;

pub struct Counter {
    window: time::Duration,
    start: Option<time::Instant>,
    count: u64,
}

impl Counter {
    pub fn new(window: time::Duration) -> Self {
        Self {
            window,
            start: None,
            count: 0,
        }
    }

    pub fn tick(&mut self) -> Option<f64> {
        if self.start.is_none() {
            self.start = Some(time::Instant::now());
            return None
        }

        self.count += 1;

        let start = self.start.unwrap();
        let now = time::Instant::now();
        let delta = now.duration_since(start);


        if now.duration_since(start) > self.window {
            let res = (self.count as f64) / (delta.as_secs_f64());
            self.start = Some(now);
            self.count = 0;
            return Some(res)
        }
        None
    }
}
