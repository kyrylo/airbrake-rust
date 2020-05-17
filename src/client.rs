
use tokio::runtime::Runtime;
use log::warn;
use hyper::{Uri, Body, Request};
use hyper::header::CONTENT_TYPE;
use hyper::client::{Client, HttpConnector};
use hyper_tls::HttpsConnector;

use crate::Notice;
use crate::NoticeBuilder;
use crate::AirbrakeConfig;

#[derive(Debug)]
pub struct AirbrakeClient {
    client: Client<HttpsConnector<HttpConnector>>,
    config: AirbrakeConfig
}

impl AirbrakeClient {
    pub fn new(config: AirbrakeConfig) -> AirbrakeClient {
        let connector = HttpsConnector::new();
        let client = Client::builder().build(connector);

        AirbrakeClient {
            client: client,
            config: config
        }
    }

    fn build_request<T>(&self, uri: Uri, payload: T) -> Request<Body>
    where T: Into<Body>
    {
        Request::post(uri)
            .header(CONTENT_TYPE, "application/json")
            .body(payload.into())
            .expect("Request creation failed unexpectedly")
    }

    async fn send_request(&self, request: Request<Body>) -> () {
        let response = self.client.request(request).await;
        match response {
            Ok ( _response ) => (),
            Err ( _x ) => warn!("notification failed")
        }
    }

    pub fn new_notice_builder(&self) -> NoticeBuilder {
        let notice_builder = match &self.config.context {
            Some( context ) => context.new_notice_builder(),
            None => Notice::builder()
        };
        notice_builder.set_client(&self)
    }

    pub fn notify(&self, mut notice: Notice) {
        // TODO: This is a codesmell- the notify function shouldn't be
        // mutating the notice. Testing this is very difficult. Too
        // difficult for me to bother figuring out, which means this is
        // poorly designed
        notice.context = notice.context.or_else(|| {self.config.context.clone()});
        let endpoint = self.config.endpoint_uri();
        let request = self.build_request(endpoint, notice);

        let mut runtime = Runtime::new().unwrap();
        runtime.block_on(self.send_request(request));
    }
}

#[cfg(test)]
mod context_user_tests {
    use std::str::FromStr;
    use std::collections::HashMap;
    use serde_json::{self, Value};
    use hyper::body::Body;
    use crate::{AirbrakeConfig, AirbrakeClient, NoticeError};

    #[test]
    fn client_with_context_included_in_notices() {
        let config = AirbrakeConfig::builder()
            .project_id("foo".to_string())
            .project_key("bar".to_string())
            .operating_system("SolarOS".to_string())
            .version("0.0.0".to_string())
            .severity("critical".to_string())
            .build()
            .unwrap();
        let client = AirbrakeClient::new(config);
        let notice = client.new_notice_builder().build();

        let expected_json = r#"
        {
            "errors": [],
            "context": {
                "notifier": {
                    "name": "airbrake-rust",
                    "url": "https://github.com/airbrake/airbrake-rust",
                    "version": "0.2.0"
                },
                "os": "SolarOS",
                "version": "0.0.0",
                "severity": "critical"
            }
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(notice)
        );
    }

    #[test]
    fn notice_from_client_inherits_context() {
        let config = AirbrakeConfig::builder()
            .project_id("foo".to_string())
            .project_key("bar".to_string())
            .operating_system("SolarOS".to_string())
            .version("0.0.0".to_string())
            .build()
            .unwrap();
        let client = AirbrakeClient::new(config);
        let notice = client.new_notice_builder()
            .severity("warning".to_string())
            .build();

        let expected_json = r#"
        {
            "errors": [],
            "context": {
                "notifier": {
                    "name": "airbrake-rust",
                    "url": "https://github.com/airbrake/airbrake-rust",
                    "version": "0.2.0"
                },
                "os": "SolarOS",
                "version": "0.0.0",
                "severity": "warning"
            }
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(notice)
        );
    }
}