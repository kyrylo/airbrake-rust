use std::thread;
use std::thread::{JoinHandle, Builder};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

use notice::Notice;
use config::Config;
use sync_sender::SyncSender;

pub struct AsyncSender {
    workers: Vec<JoinHandle<()>>,
    tx: Sender<Option<Notice>>,
}

impl AsyncSender {
    pub fn new(config: &Config) -> AsyncSender {
        let (tx, rx) = channel();
        let rx = Arc::new(Mutex::new(rx));

        AsyncSender {
            workers: AsyncSender::spawn_workers(config, rx),
            tx: tx,
        }
    }

    pub fn send(&self, notice: Notice) {
        self.tx.send(Some(notice)).unwrap();
    }

    pub fn close(&mut self) {
        for _ in 0..self.workers.len() {
            self.tx.send(None).unwrap();
        }

        for _ in 0..self.workers.len() {
            let worker = self.workers.pop().unwrap();
            worker.join().unwrap();
        }
    }

    fn spawn_workers(config: &Config,
                     rx: Arc<Mutex<Receiver<Option<Notice>>>>) -> Vec<JoinHandle<()>> {
        let mut workers = vec![];

        for i in 0..config.workers {
            let sync_sender = SyncSender::new(config);
            let rx = rx.clone();

            workers.push(Builder::new().name(i.to_string()).spawn(move || {
                debug!("**Airbrake: spawning thread {:?}", thread::current());

                loop {
                    let message = {
                        let lock = rx.lock().unwrap();
                        lock.recv()
                    };

                    match message {
                        Ok(Some(notice)) => {
                            debug!("**Airbrake: sending {:?}", notice);
                            sync_sender.send(notice);
                        },
                        Ok(None) | Err(..) => {
                            debug!("**Airbrake: terminating thread {:?}", thread::current());
                            break;
                        },
                    };
                }
            }).unwrap());
        }

        workers
    }
}
