use reqwest;
use serde_json;

use std::error::Error;

use notice::Notice;

#[derive(Debug, Default)]
pub struct Notifier<'a> {
    config: Config<'a>,
}

#[derive(Debug, Default)]
pub struct Config<'a> {
    pub project_id: u32,
    pub project_key: &'a str,
    pub proxy_url: Option<String>,
}

impl<'a> Notifier<'a> {
    pub fn new(config: Config<'a>) -> Self {
        Self { config: config }
    }

    pub fn notify(&self, notice: Notice) -> Result<reqwest::Response, reqwest::Error> {
        let mut client_builder = reqwest::Client::builder();
        if self.config.proxy_url.is_some() {
            client_builder.proxy(reqwest::Proxy::all(
                self.config.proxy_url.to_owned().unwrap().as_str(),
            )?);
        }

        client_builder
            .build()?
            .post(&format!(
                "https://airbrake.io/api/v3/projects/{}/notices",
                self.config.project_id
            ))
            .header(reqwest::header::Authorization(reqwest::header::Bearer {
                token: self.config.project_key.to_owned(),
            }))
            .body(serde_json::to_string(&notice).unwrap())
            .send()
    }

    pub fn build_notice<T: Error>(&self, error: T) -> Notice {
        Notice::new(error)
    }
}
