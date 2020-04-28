use std::error::Error;

use serde_json::Value;
use tokio::runtime::Runtime;
use crate::config::Config;
use crate::notice::Notice;
use crate::async_sender::AsyncSender;

pub struct Notifier {
    async_sender: AsyncSender,
    config: Config,
}

impl Notifier {
    pub fn new(config: Config) -> Notifier {
        Notifier {
            async_sender: AsyncSender::new(&config),
            config: config,
        }
    }

    // TODO: Should not panic on closed notifier
    pub fn notify<E: Error>(&self, error: E) {
        let notice = Notice::new(&self.config, error);
        Runtime::new().unwrap().block_on(self.async_sender.send(notice));
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
}
