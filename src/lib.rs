extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Notifier {
    config: Config,
}

#[derive(Debug, Default)]
pub struct Config {
    pub project_id: u32,
    pub project_key: String,
}

#[derive(Serialize)]
struct Error {
    #[serde(rename = "type")]
    type_: String,
    message: String,
}

#[derive(Serialize)]
pub struct Notice {
    errors: Vec<Error>,
    params: HashMap<String, Param>,
}

#[derive(Serialize)]
pub enum Param {
    Int32(i32),
    String(String),
}

impl Notice {
    pub fn new<T: std::error::Error>(error: T, params: Option<HashMap<String, Param>>) -> Self {
        Self {
            errors: vec![
                Error {
                    type_: format!("{:?}", error)
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .to_owned(),
                    message: String::from(error.description()),
                },
            ],
            params: params.unwrap_or(HashMap::new()),
        }
    }
}

impl Notifier {
    pub fn new(config: Config) -> Self {
        Self { config: config }
    }

    pub fn notify<T: std::error::Error>(
        &self,
        error: T,
        params: Option<HashMap<String, Param>>,
    ) -> reqwest::Response {
        let notice = self.build_notice(error, params);

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

    pub fn build_notice<T: std::error::Error>(
        &self,
        error: T,
        params: Option<HashMap<String, Param>>,
    ) -> Notice {
        Notice::new(error, params)
    }
}
