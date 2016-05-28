use std::error::Error;

use hyper::Url;
use hyper::header::ContentType;
use hyper::client::{Client, Body};

use config::Config;
use notice::Notice;

pub struct Notifier {
    config: Config,
}

impl Notifier {
    pub fn new(config: Config) -> Notifier {
        Notifier { config: config }
    }

    pub fn notify_sync<E: Error>(&self, error: E) {
        let notice = Notice::new(error);
        self.send_notice(notice);
    }

    fn send_notice(&self, notice: Notice) {
        let client = Client::new();
        let uri = Url::parse(&self.config.endpoint()).ok().expect("malformed URL");

        let payload = notice.to_json();
        let bytes = payload.as_bytes();

        let response = client.post(uri)
            .header(ContentType::json())
            .body(Body::BufBody(bytes, bytes.len()))
            .send();

        println!("{:?}", response);
    }
}
