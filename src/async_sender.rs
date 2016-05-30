use notice::Notice;
use config::Config;
use sync_sender::SyncSender;

pub struct AsyncSender {
    sync_sender: SyncSender
}

impl AsyncSender {
    pub fn new(config: &Config) -> AsyncSender {
        AsyncSender {
            sync_sender: SyncSender::new(&config)
        }
    }

    pub fn send(&self, notice: Notice) {
        self.sync_sender.send(notice);
    }
}
