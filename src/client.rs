use std::error::Error;

use tokio::runtime::Runtime;
use log::warn;
use hyper::{Uri, Body, Request};
use hyper::header::CONTENT_TYPE;
use hyper::client::{Client, HttpConnector};
use hyper_tls::HttpsConnector;
use crate::Notice;
use crate::AirbrakeConfig;


pub struct AirbrakeClient {
    client: Client<HttpsConnector<HttpConnector>>,
    config: AirbrakeConfig
}

impl AirbrakeClient {
    pub fn new(config: AirbrakeConfig) -> AirbrakeClient {
        let connector = HttpsConnector::new();
        let client = Client::builder().build(connector);

        AirbrakeClient {
            client: client,
            config: config
        }
    }

    async fn send<T>(&self, uri: Uri, payload: T) -> ()
    where T: Into<Body>
    {
        let request = Request::post(uri)
            .header(CONTENT_TYPE, "application/json")
            .body(payload.into())
            .unwrap();
        let response = self.client
            .request(request)
            .await
            .unwrap();
        if response.status() == 200 {
            warn!("notification failed")
        }
    }

    pub fn notify<E: Error>(&self, error: E) {
        let endpoint = self.config.endpoint_uri();
        let payload = Notice::new(&self.config, error);
        let mut runtime = Runtime::new().unwrap();
        runtime.block_on(self.send(endpoint, payload));
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
