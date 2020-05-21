
use std::time::Instant;
use log::warn;
use reqwest::blocking::Client;
use serde::Serialize;

use crate::Notice;
use crate::NoticeBuilder;
use crate::AirbrakeConfig;

#[derive(Debug)]
pub struct AirbrakeClient {
    client: Client,
    config: AirbrakeConfig
}

impl AirbrakeClient {
    pub fn new(config: AirbrakeConfig) -> AirbrakeClient {
        AirbrakeClient {
            client: Client::new(),
            config: config
        }
    }

    fn send_request<T>(&self, uri: &str, payload: &T) -> ()
    where T: Serialize
    {
        // Prepare a duration timer to track how long it takes to send the request.
        let start_time = Instant::now();

        // Now send the request to the airbrake server
        let response = self.client.post(uri)
            .json(payload)
            .send();

        // Calculate send duration and print it to debug
        let duration = start_time.elapsed();
        debug!("Airbrake notify request took: {:?}", duration);

        // Now handle the response
        match response {
            Ok ( _ ) => (),
            Err ( _ ) => warn!("notification failed")
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
        self.send_request(&endpoint, &notice)
    }
}

#[cfg(test)]
mod context_user_tests {
    use std::str::FromStr;
    use std::collections::HashMap;
    use serde_json::{self, Value};
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