extern crate futures;
extern crate reqwest;
#[macro_use]
extern crate serde_json;
use reqwest::StatusCode;
use reqwest::header::{Authorization, Bearer};
// use futures::Future;

#[derive(Debug)]
pub struct Notifier {
    config: Config,
}

#[derive(Debug, Default)]
pub struct Config {
    pub project_id: u32,
    pub project_key: String,
}

impl Notifier {
    pub fn new(config: Config) -> Self {
        Self { config: config }
    }

    pub fn notify(&self, error: String) -> StatusCode {
        let client = reqwest::Client::new();

        let url = format!(
            "https://airbrake.io/api/v3/projects/{}/notices",
            self.config.project_id
        );
        let resp = client
            .post(&url)
            .header(Authorization(Bearer {
                token: self.config.project_key.to_owned(),
            }))
            .body(
                json!({
                    "errors": [
                        {
                            "type": "error1",
                            "message": error,
                            "backtrace": [
                                {
                                    "file": "backtrace file",
                                    "line": 10,
                                    "function": "backtrace function"
                                }
                            ]
                        }
                    ]
                }).to_string(),
            )
            .send()
            .unwrap();

        resp.status()
    }
}
