use log::warn;
use serde_json;
use hyper::{Uri, Request};
use hyper::header::CONTENT_TYPE;
use hyper::client::{Client, HttpConnector};
use hyper_tls::HttpsConnector;

use crate::config::Config;
use crate::notice::Notice;

#[derive(Debug)]
pub struct AsyncSender {
    client: Client<HttpsConnector<HttpConnector>>,
    endpoint: Uri,
}

impl AsyncSender {
    pub fn new(config: &Config) -> AsyncSender {
        let https = HttpsConnector::new();
        let client = Client::builder().build(https);
        // let client = if config.proxy.is_empty() {
        //     let https = HttpsConnector::new();
        //     Client::builder().build(https)
        // } else {
        //     let mut proxy = config.proxy.clone();
        //     let mut port = 80;

        //     if let Some(colon) = proxy.rfind(':') {
        //         port = proxy[colon + 1..].parse().unwrap_or_else(|e| {
        //             panic!("proxy is malformed: {:?}, port parse error: {}",
        //                    proxy, e);
        //         });
        //         proxy.truncate(colon);
        //     }
        //     Client::with_http_proxy(proxy, port)
        // };

        AsyncSender {
            client: client,
            endpoint: config.endpoint_uri(),
        }
    }

    pub async fn send(&self, notice: Notice) -> () {
        let request = Request::post(&self.endpoint)
            .header(CONTENT_TYPE, "application/json")
            .body(notice.into())
            .unwrap();
        let response = self.client
            .request(request)
            .await
            .unwrap();
        if response.status() == 200 {
            warn!("notification failed")
        }
    }
}
