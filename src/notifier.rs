use std::error::Error;

use config::Config;
use notice::Notice;
use sync_sender::SyncSender;
use async_sender::AsyncSender;

pub struct Notifier {
    sync_sender: SyncSender,
    async_sender: AsyncSender,
}

impl Notifier {
    pub fn new(config: Config) -> Notifier {
        Notifier {
            sync_sender: SyncSender::new(&config),
            async_sender: AsyncSender::new(&config),
        }
    }

    pub fn notify<E: Error>(&self, error: E) {
        let notice = Notice::new(error);
        self.async_sender.send(notice);
    }

    pub fn notify_sync<E: Error>(&self, error: E) {
        let notice = Notice::new(error);
        self.sync_sender.send(notice);
    }

    pub fn close(&mut self) {
        self.async_sender.close();
    }
}
