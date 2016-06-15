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
    environment: Option<Json>,
    params: Option<Json>,
}

#[derive(Debug, RustcEncodable)]
struct Context {
    notifier: NotifierPayload,
}

#[derive(Debug, RustcEncodable)]
struct NotifierPayload {
    name: String,
    version: String,
    url: String,
}

#[derive(Debug, RustcEncodable)]
pub struct BacktraceLine {
    pub file: String,
    pub function: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

impl ToJson for BacktraceLine {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("file".to_owned(), self.file.to_json());
        d.insert("function".to_owned(), self.function.to_json());
        d.insert("line".to_owned(), self.line.to_json());
        d.insert("column".to_owned(), self.column.to_json());
        Json::Object(d)
    }
}

#[derive(Debug)]
struct AirbrakeError {
    type_: String,
    message: String,
    backtrace: Vec<BacktraceLine>,
}

impl ToJson for AirbrakeError {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("type".to_owned(), self.type_.to_json());
        d.insert("message".to_owned(), self.message.to_json());
        d.insert("backtrace".to_owned(), self.backtrace.to_json());
        Json::Object(d)
    }
}

impl Notice {
    pub fn new<E: Error>(error: E) -> Notice {
        let mut dummy = BTreeMap::new();
        dummy.insert("something".to_owned(), "anything".to_json());

        let dummy_btrace = BacktraceLine {
            file: "some_file".to_owned(),
            function: None,
            line: None,
            column: None,
        };

        Notice {
            errors: vec![AirbrakeError {
                             type_: format!("{:?}", error)
                                        .split_whitespace()
                                        .next()
                                        .unwrap()
                                        .to_owned(),
                             message: format!("{}", error),
                             backtrace: vec![dummy_btrace],
                         }
                         .to_json()],
            context: Context {
                notifier: NotifierPayload {
                    name: NOTIFIER_NAME.to_owned(),
                    version: env!("CARGO_PKG_VERSION").to_owned(),
                    url: NOTIFIER_URL.to_owned(),
                },
            },
            environment: Some(Json::Object(dummy)),
            params: None,
        }
    }

    pub fn to_json(&self) -> String {
        json::encode(&self).unwrap()
    }
}
