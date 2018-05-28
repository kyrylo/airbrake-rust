use reqwest;
use serde_json;

use std::error::Error;

use notice::Notice;

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

    pub fn notify(&self, notice: Notice) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!(
                "https://airbrake.io/api/v3/projects/{}/notices",
                self.config.project_id
            ))
            .header(reqwest::header::Authorization(reqwest::header::Bearer {
                token: self.config.project_key.to_owned(),
            }))
            .body(serde_json::to_string(&notice).unwrap())
            .send()
            .unwrap()
    }

    pub fn build_notice<T: Error>(&self, error: T) -> Notice {
        Notice::new(error)
    }
}
