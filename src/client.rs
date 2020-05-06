
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

    fn request<T>(&self, uri: Uri, payload: T) -> Request<Body>
    where T: Into<Body>
    {
        Request::post(uri)
            .header(CONTENT_TYPE, "application/json")
            .body(payload.into())
            .expect("Request creation failed unexpectedly")
    }

    async fn send(&self, request: Request<Body>) -> () {
        let response = self.client.request(request).await;
        match response {
            Ok ( response ) => (),
            Err ( x ) => warn!("notification failed")
        }
    }

    pub fn notify(&self, notice: Notice) {
        let endpoint = self.config.endpoint_uri();
        let request = self.request(endpoint, notice);

        let mut runtime = Runtime::new().unwrap();
        runtime.block_on(self.send(request));
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
