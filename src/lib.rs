extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

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
struct Notice {
    errors: Vec<Error>,
}

impl Notice {
    pub fn new<T: std::error::Error>(error: T) -> Self {
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
        }
    }
}

impl Notifier {
    pub fn new(config: Config) -> Self {
        Self { config: config }
    }

    pub fn notify<T: std::error::Error>(&self, error: T) -> reqwest::Response {
        reqwest::Client::new()
            .post(&self.endpoint())
            .header(reqwest::header::Authorization(reqwest::header::Bearer {
                token: self.config.project_key.to_owned(),
            }))
            .body(serde_json::to_string(&Notice::new(error)).unwrap())
            .send()
            .unwrap()
    }

    fn endpoint(&self) -> String {
        format!(
            "https://airbrake.io/api/v3/projects/{}/notices",
            self.config.project_id
        )
    }
}
