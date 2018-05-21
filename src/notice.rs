use std::error::Error;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Notice {
    errors: Vec<AirbrakeError>,
    params: HashMap<String, Param>,
}

#[derive(Serialize)]
struct AirbrakeError {
    #[serde(rename = "type")]
    type_: String,
    message: String,
}

#[derive(Serialize)]
pub enum Param {
    Int32(i32),
    String(String),
}

impl Notice {
    pub fn new<T: Error>(error: T, params: Option<HashMap<String, Param>>) -> Self {
        Self {
            errors: vec![
                AirbrakeError {
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
