use serde_json;

use std::error::Error;
use hyper::body::Body;
use crate::config::Config;

const NOTIFIER_NAME: &'static str = "airbrake-rust";
const NOTIFIER_URL: &'static str = "https://github.com/airbrake/airbrake-rust";

#[derive(Debug, Serialize)]
pub struct Notice {
    errors: Vec<AirbrakeError>,
    context: Context,
}

#[derive(Debug, Serialize)]
pub struct Context {
    notifier: NotifierPayload,
    version: String,
}

#[derive(Debug, Serialize)]
pub struct NotifierPayload {
    name: String,
    version: String,
    url: String,
}

#[derive(Debug, Serialize)]
pub struct AirbrakeError {
    #[serde(rename="type")]
    type_: String,
    message: String,
}

impl Notice {
    pub fn new<E: Error>(config: &Config, error: E) -> Notice {
        Notice {
            errors: vec![
                AirbrakeError {
                    type_: format!("{:?}", error).split_whitespace().next().unwrap().to_owned(),
                    message: format!("{}", error),
                }
            ],
            context: Context {
                notifier: NotifierPayload {
                    name: NOTIFIER_NAME.to_owned(),
                    version: env!("CARGO_PKG_VERSION").to_owned(),
                    url: NOTIFIER_URL.to_owned(),
                },
                version: config.app_version.clone(),
            },
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Into<Body> for Notice {
    fn into(self) -> Body {
        Body::from(self.to_json())
    }
}
