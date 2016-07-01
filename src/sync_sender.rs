use std::io::Read;

use hyper::Url;
use hyper::header::ContentType;
use hyper::client::{Client, Body};
use rustc_serialize::json::Json;

use config::Config;
use notice::Notice;

#[derive(Debug)]
pub struct SyncSender {
    client: Client,
    endpoint: String,
}

impl SyncSender {
    pub fn new(config: &Config) -> SyncSender {
        let client = if config.proxy.is_empty() {
            Client::new()
        } else {
            let mut proxy = config.proxy.clone();
            let mut port = 80;

            if let Some(colon) = proxy.rfind(':') {
                port = proxy[colon + 1..].parse().unwrap_or_else(|e| {
                    panic!("proxy is malformed: {:?}, port parse error: {}",
                           proxy, e);
                });
                proxy.truncate(colon);
            }
            Client::with_http_proxy(proxy, port)
        };

        SyncSender {
            client: client,
            endpoint: config.endpoint(),
        }
    }

    pub fn send(&self, notice: Notice) -> Json {
        let uri = Url::parse(&self.endpoint).ok().expect("malformed URL");

        let payload = notice.to_json();
        let bytes = payload.as_bytes();

        debug!("**Airbrake: sending {}", payload);

        let response = self.client.post(uri)
            .header(ContentType::json())
            .body(Body::BufBody(bytes, bytes.len()))
            .send();

        let mut buffer = String::new();
        response.unwrap().read_to_string(&mut buffer).unwrap();
        Json::from_str(&buffer).unwrap()
    }
}
