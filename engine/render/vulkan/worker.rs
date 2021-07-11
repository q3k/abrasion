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

use log;

use std::sync::mpsc;
use std::thread;

enum Command {
    Exit,
}

pub struct Worker {
    handle: Option<thread::JoinHandle<()>>,
    control: mpsc::Sender<Command>,
}

impl Worker {
    pub fn new(id: u64) -> Worker {
        let (control, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            log::info!("Worker {} starting...", id);
            loop {
                let mut done = false;

                match rx.recv() {
                    Err(err) => {
                        log::error!("Worker {} cannot receive, dying: {}", id, err);
                        done = true;
                    },
                    Ok(cmd) => {
                        match cmd {
                            Command::Exit => {
                                log::info!("Worker {} exiting", id);
                                done = true;
                            }
                        }
                    },
                }

                if done {
                    break
                }
            }
        });

        Worker {
            handle: Some(handle),
            control
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.control.send(Command::Exit).unwrap();
        self.handle.take().unwrap().join().unwrap();
    }
}
