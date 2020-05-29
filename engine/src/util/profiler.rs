use std::time;
use std::vec::Vec;

pub struct Profiler {
    sections: Vec<(String, time::Duration)>,
    cur: time::Instant,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            cur: time::Instant::now(),
        }
    }

    pub fn end(&mut self, name: &str) {
        let now = time::Instant::now();
        self.sections.push((name.to_string(), now.duration_since(self.cur)));
        self.cur = now;
    }

    pub fn print(&self) {
        let total: f64 = self.sections.iter().map(|(_, d)| d.as_secs_f64()).sum();
        for (n, d) in &self.sections {
            log::info!("{}: {:.5}%", n, 100.0*d.as_secs_f64()/total);
        }
    }
}
