use reqwest;
use serde_json;

use std::error::Error;

use notice::Notice;

const AIRBRAKE_API: &'static str = "https://airbrake.io";

#[derive(Default)]
pub struct Notifier<'a> {
    config: Config<'a>,
    filters: Vec<Box<Fn(&mut Notice) + 'a>>,
}

pub struct Config<'a> {
    pub project_id: u32,
    pub project_key: &'a str,
    pub proxy_url: &'a str,
    pub host: &'a str,
    pub app_version: &'a str,
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self {
            project_id: 0,
            project_key: "",
            proxy_url: "",
            host: AIRBRAKE_API,
            app_version: "",
        }
    }
}

impl<'a> Notifier<'a> {
    pub fn new(config: Config<'a>) -> Self {
        Self {
            config: config,
            filters: Vec::new(),
        }
    }

    pub fn notify(&self, notice: &mut Notice) -> Result<reqwest::Response, reqwest::Error> {
        for filter in &self.filters {
            filter(notice);
        }

        let mut client_builder = reqwest::Client::builder();
        if !self.config.proxy_url.is_empty() {
            client_builder.proxy(reqwest::Proxy::all(self.config.proxy_url)?);
        }

        let mut headers = reqwest::header::Headers::new();
        headers.set(reqwest::header::ContentType::json());
        headers.set(reqwest::header::Authorization(reqwest::header::Bearer {
            token: String::from(self.config.project_key),
        }));

        client_builder
            .build()?
            .post(&format!(
                "{}/api/v3/projects/{}/notices",
                self.config.host, self.config.project_id
            ))
            .headers(headers)
            .body(serde_json::to_string(&notice).unwrap())
            .send()
    }

    pub fn build_notice<T: Error>(&self, error: T) -> Notice {
        let mut notice = Notice::new(error);
        notice.set_app_version(self.config.app_version);
        notice
    }

    pub fn add_filter<T: Fn(&mut Notice) + 'a>(&mut self, filter: T) {
        self.filters.push(Box::new(filter));
    }
}
