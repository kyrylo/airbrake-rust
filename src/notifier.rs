use std::error::Error;

use serde_json::Value;

use config::Config;
use notice::Notice;
use sync_sender::SyncSender;
use async_sender::AsyncSender;

pub struct Notifier {
    sync_sender: SyncSender,
    async_sender: AsyncSender,
    closed: bool,
    config: Config,
}

impl Notifier {
    pub fn new(config: Config) -> Notifier {
        Notifier {
            sync_sender: SyncSender::new(&config),
            async_sender: AsyncSender::new(&config),
            closed: false,
            config: config,
        }
    }

    pub fn notify<E: Error>(&self, error: E) {
        if self.closed {
            panic!("attempted to send through a closed Airbrake notifier");
        }

        let notice = Notice::new(&self.config, error);
        self.async_sender.send(notice);
    }

    pub fn notify_sync<E: Error>(&self, error: E) -> Value {
        if self.closed {
            panic!("attempted to send through a closed Airbrake notifier");
        }

        let notice = Notice::new(&self.config, error);
        self.sync_sender.send(notice)
    }

    pub fn close(&mut self) {
        if self.closed {
            panic!("attempted to close an already closed Airbrake notifier");
        }

        self.async_sender.close();
        self.closed = true;
    }
}

#[cfg(test)]
mod tests {
    use std::io::Error;
    use super::Notifier;
    use config::Config;

    #[test]
    fn close_doesnt_panic() {
        let mut notifier = Notifier::new(Config::new());
        notifier.close();
    }

    #[test]
    #[should_panic(expected="attempted to close an already closed Airbrake notifier")]
    fn double_close_panics() {
        let mut notifier = Notifier::new(Config::new());
        notifier.close();
        notifier.close();
    }

    #[test]
    #[should_panic(expected="attempted to send through a closed Airbrake notifier")]
    fn notify_with_closed_notifier_panics() {
        let mut notifier = Notifier::new(Config::new());
        notifier.close();
        notifier.notify(Error::last_os_error());
    }

    #[test]
    #[should_panic(expected="attempted to send through a closed Airbrake notifier")]
    fn notify_sync_with_closed_notifier_panics() {
        let mut notifier = Notifier::new(Config::new());
        notifier.close();
        notifier.notify_sync(Error::last_os_error());
    }
}
