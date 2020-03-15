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
