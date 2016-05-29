use std::error::Error;
use std::collections::BTreeMap;

use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};

const NOTIFIER_NAME: &'static str = "airbrake-rust";
const NOTIFIER_URL: &'static str = "https://github.com/airbrake/airbrake-rust";

#[derive(Debug, RustcEncodable)]
pub struct Notice {
    errors: Vec<Json>,
    context: Context,
}

#[derive(Debug, RustcEncodable)]
struct Context {
    notifier: NotifierPayload
}

#[derive(Debug, RustcEncodable)]
struct NotifierPayload {
    name: String,
    version: String,
    url: String,
}

#[derive(Debug)]
struct AirbrakeError {
    type_: String,
    message: String,
}

impl ToJson for AirbrakeError {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("type".to_owned(), self.type_.to_json());
        d.insert("message".to_owned(), self.message.to_json());
        Json::Object(d)
    }
}

impl Notice {
    pub fn new<E: Error>(error: E) -> Notice {
        Notice {
            errors: vec![
                AirbrakeError {
                    type_: format!("{:?}", error).split_whitespace().next().unwrap().to_owned(),
                    message: format!("{}", error),
                }.to_json()
            ],
            context: Context {
                notifier: NotifierPayload {
                    name: NOTIFIER_NAME.to_owned(),
                    version: env!("CARGO_PKG_VERSION").to_owned(),
                    url: NOTIFIER_URL.to_owned(),
                }
            },
        }
    }

    pub fn to_json(&self) -> String {
        json::encode(&self).unwrap()
    }
}
