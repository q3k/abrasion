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
