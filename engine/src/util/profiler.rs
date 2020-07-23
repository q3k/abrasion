// Copyright 2020 Sergiusz 'q3k' Bazanski <q3k@q3k.org>
//
// This file is part of Abrasion.
//
// Abrasion is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, version 3.
//
// Abrasion is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// Abrasion.  If not, see <https://www.gnu.org/licenses/>.

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
            log::debug!("{}: {:.5}%", n, 100.0*d.as_secs_f64()/total);
        }
    }
}
