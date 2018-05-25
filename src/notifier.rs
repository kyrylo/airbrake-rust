use reqwest;
use serde_json;
use backtrace::Backtrace;

use std::error::Error;
use std::collections::HashMap;

use notice::{Notice, Param};

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

    pub fn notify<T: Error>(
        &self,
        error: T,
        params: Option<HashMap<String, Param>>,
        backtrace: Option<Backtrace>,
    ) -> reqwest::Response {
        let notice = self.build_notice(error, params, backtrace);

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

    pub fn build_notice<T: Error>(
        &self,
        error: T,
        params: Option<HashMap<String, Param>>,
        backtrace: Option<Backtrace>,
    ) -> Notice {
        Notice::new(error, params, backtrace)
    }
}
