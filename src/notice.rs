use std::error::Error;
use std::collections::BTreeMap;
use std::collections::HashMap;

use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};

#[derive(Debug, RustcEncodable)]
pub struct Notice<'a> {
    errors: Vec<Json>,
    context: HashMap<&'a str, HashMap<&'a str, &'a str>>,
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

impl<'a> Notice<'a> {
    pub fn new<E: Error>(error: E) -> Notice<'a> {
        let mut context = HashMap::new();
        let mut notifier = HashMap::new();
        notifier.insert("name", "airbrake-rust");
        notifier.insert("version", "0.0.1");
        notifier.insert("url", "https://github.com/airbrake/airbrake-rust");

        context.insert("notifier", notifier);

        Notice {
            errors: vec![
                AirbrakeError {
                    type_: format!("{:?}", error).split_whitespace().next().unwrap().to_owned(),
                    message: format!("{}", error),
                }.to_json()
            ],
            context: context,
        }
    }

    pub fn to_json(&self) -> String {
        json::encode(&self).unwrap()
    }
}
