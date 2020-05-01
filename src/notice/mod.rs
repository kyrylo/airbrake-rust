
mod context;
mod error;

pub use context::ContextNotifier;
pub use context::ContextUser;
pub use context::Context;
pub use context::CONTEXT_NOTIFIER;
pub use error::NoticeError;
pub use error::NoticeBacktrace;
pub use error::BacktraceCodeBlock;

use serde_json;

use std::error::Error;
use std::collections::HashMap;
use std::string::ToString;
use hyper::body::Body;
use crate::AirbrakeConfig;

#[derive(Debug, Serialize)]
pub struct Notice {
    errors: Vec<NoticeError>,

    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<Context>,

    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    session: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<HashMap<String, String>>
}

impl Notice {
    pub fn new<E: Error>(config: &AirbrakeConfig, error: E) -> Notice {
        Notice {
            errors: vec![
                NoticeError {
                    type_: format!("{:?}", error).split_whitespace().next().unwrap().to_owned(),
                    message: Some(format!("{}", error)),
                    backtrace: None
                }
            ],
            context: Some(Context {
                notifier: &CONTEXT_NOTIFIER,
                operating_system: None,
                hostname: None,
                language: None,
                environment: None,
                severity: None,
                version: Some(config.app_version.clone()),
                url: None,
                root_directory: None,
                user: None,
                route: None,
                http_method: None
            }),
            environment: None,
            session: None,
            params: None
        }
    }
}

impl Into<Body> for Notice {
    fn into(self) -> Body {
        Body::from(serde_json::json!(self).to_string())
    }
}
