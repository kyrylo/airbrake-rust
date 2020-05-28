use log::warn;
use reqwest::blocking::Client;
use serde::Serialize;
use std::env;
use std::marker::{Send, Sync};
use std::panic::PanicInfo;
use std::time::Instant;

use crate::Notice;
use crate::NoticeBuilder;
use crate::NoticeError;
use crate::{Context, ContextBuilder, ContextProperties};

const DEFAULT_HOSTNAME: &str = "https://airbrake.io";
const ENV_VAR_PROJECT_ID: &str = "AIRBRAKE_PROJECT_ID";
const ENV_VAR_PROJECT_KEY: &str = "AIRBRAKE_API_KEY";
const ENV_VAR_HOST: &str = "AIRBRAKE_HOST";

#[derive(Debug, PartialEq)]
pub enum AirbrakeClientBuilderError {
    MissingProjectId,
    MissingProjectKey,
    EmptyProjectId,
    EmptyProjectKey,
}

#[derive(Default)]
pub struct AirbrakeClientBuilder {
    pub project_id: Option<String>,
    pub project_key: Option<String>,
    pub host: Option<String>,
    pub proxy: Option<String>,
    pub context: Option<ContextBuilder>,
}

impl AirbrakeClientBuilder {
    pub fn new() -> AirbrakeClientBuilder {
        AirbrakeClientBuilder::default()
    }

    pub fn configure<F>(&mut self, builder_callback: F) -> &mut AirbrakeClientBuilder
    where
        F: Fn(&mut AirbrakeClientBuilder),
    {
        builder_callback(self);
        self
    }

    pub fn project<'a>(
        &'a mut self,
        project_id: &str,
        project_key: &str,
    ) -> &'a mut AirbrakeClientBuilder {
        self.project_id(project_id).project_key(project_key)
    }

    pub fn project_id<'a>(&'a mut self, project_id: &str) -> &'a mut AirbrakeClientBuilder {
        self.project_id = Some(project_id.to_string());
        self
    }

    /// Builder can pull env vars from environment
    ///
    /// ```
    /// use std::env;
    /// use airbrake::AirbrakeClient;
    ///
    /// // Set vars to set up our test
    /// env::set_var("AIRBRAKE_PROJECT_ID", "foo");
    /// env::set_var("AIRBRAKE_API_KEY", "bar");
    ///
    /// // Now build the config using just the environment variables
    /// let config = AirbrakeClient::builder()
    ///     .project_id_from_env().expect("missing id")
    ///     .project_key_from_env().expect("missing key")
    ///     .build();
    /// ```
    ///
    /// If you wanted to handle more complex logic around the
    /// environment variables
    ///
    /// ```
    /// use std::env;
    /// use airbrake::AirbrakeClient;
    ///
    /// // Only set the project key using the env var
    /// env::set_var("AIRBRAKE_API_KEY", "baz");
    ///
    /// // Begin constructing a client
    /// let mut client_builder = AirbrakeClient::builder();
    /// if client_builder.project_id_from_env().is_err() {
    ///     client_builder.project_id("foo");
    /// }
    /// if client_builder.project_key_from_env().is_err() {
    ///     client_builder.project_key("bar");
    /// }
    /// let config = client_builder.build().unwrap();
    /// ```
    ///
    pub fn project_id_from_env(&mut self) -> Result<&'_ mut AirbrakeClientBuilder, env::VarError> {
        match env::var(ENV_VAR_PROJECT_ID) {
            Ok(val) => {
                self.project_id = Some(val);
                Ok(self)
            }
            Err(e) => Err(e),
        }
    }

    pub fn project_key_from_env(&mut self) -> Result<&'_ mut AirbrakeClientBuilder, env::VarError> {
        match env::var(ENV_VAR_PROJECT_KEY) {
            Ok(val) => {
                self.project_key = Some(val);
                Ok(self)
            }
            Err(e) => Err(e),
        }
    }

    pub fn project_key(&mut self, project_key: &str) -> &'_ mut AirbrakeClientBuilder {
        self.project_key = Some(project_key.to_string());
        self
    }

    pub fn host_from_env(&mut self) -> Result<&'_ mut AirbrakeClientBuilder, env::VarError> {
        match env::var(ENV_VAR_HOST) {
            Ok(val) => {
                self.host = Some(val);
                Ok(self)
            }
            Err(e) => Err(e),
        }
    }

    pub fn host(&mut self, host: &str) -> &'_ mut AirbrakeClientBuilder {
        self.host = Some(host.to_string());
        self
    }

    pub fn proxy(&mut self, proxy: &str) -> &'_ mut AirbrakeClientBuilder {
        self.proxy = Some(proxy.to_string());
        self
    }

    // Sets the configurations context to an existing context builder
    pub fn context(&mut self, context: ContextBuilder) -> &'_ mut AirbrakeClientBuilder {
        self.context = Some(context);
        self
    }

    pub fn build(&self) -> Result<AirbrakeClient, AirbrakeClientBuilderError> {
        let project_id = match &self.project_id {
            Some(id) => id,
            None => return Err(AirbrakeClientBuilderError::MissingProjectId),
        };
        let project_key = match &self.project_key {
            Some(key) => key,
            None => return Err(AirbrakeClientBuilderError::MissingProjectKey),
        };
        if project_id.is_empty() {
            return Err(AirbrakeClientBuilderError::EmptyProjectId);
        }
        if project_key.is_empty() {
            return Err(AirbrakeClientBuilderError::EmptyProjectKey);
        }
        let context = self.context.clone().map(|c| c.build());

        Ok(AirbrakeClient {
            client: Client::new(),
            project_id: project_id.to_string(),
            project_key: project_key.to_string(),
            host: self
                .host
                .clone()
                .unwrap_or_else(|| DEFAULT_HOSTNAME.to_owned()),
            proxy: self.proxy.clone(),
            context,
        })
    }
}

impl ContextProperties for AirbrakeClientBuilder {
    fn get_context(&self) -> Option<ContextBuilder> {
        self.context.clone()
    }

    fn set_context(&mut self, context: ContextBuilder) -> &'_ mut Self {
        self.context(context)
    }
}

#[derive(Debug)]
pub enum AirbrakeClientError {
    ReqwestError(reqwest::Error),
    NoticeClientNotSet,
}

impl From<reqwest::Error> for AirbrakeClientError {
    fn from(err: reqwest::Error) -> AirbrakeClientError {
        AirbrakeClientError::ReqwestError(err)
    }
}

#[derive(Debug, Clone)]
pub struct AirbrakeClient {
    client: Client,
    project_id: String,
    project_key: String,
    host: String,
    proxy: Option<String>,
    context: Option<Context>,
}

impl AirbrakeClient {
    pub fn builder() -> AirbrakeClientBuilder {
        AirbrakeClientBuilder::new()
    }

    fn endpoint_uri(&self) -> String {
        format!(
            "{}/api/v3/projects/{}/notices?key={}",
            self.host, self.project_id, self.project_key,
        )
    }

    fn send_request<T>(&self, uri: &str, payload: &T) -> Result<(), AirbrakeClientError>
    where
        T: Serialize,
    {
        // Prepare a duration timer to track how long it takes to send the request.
        let start_time = Instant::now();

        // Now send the request to the airbrake server
        let response = self.client.post(uri).json(payload).send();

        // Calculate send duration and print it to debug
        let duration = start_time.elapsed();
        debug!("Airbrake notify request took: {:?}", duration);

        // Return the response, ignoring the content if it's successful
        response.map(|_| ()).map_err(|e| {
            warn!("Airbrake notification failed");
            AirbrakeClientError::from(e)
        })
    }

    pub fn new_notice_builder(&self) -> NoticeBuilder {
        let mut notice_builder = match &self.context {
            Some(context) => context.new_notice_builder(),
            None => Notice::builder(),
        };
        notice_builder.set_client(&self);
        notice_builder
    }

    pub fn notify(&self, mut notice: Notice) -> Result<(), AirbrakeClientError> {
        // TODO: This is a codesmell- the notify function shouldn't be
        // mutating the notice. Testing this is very difficult. Too
        // difficult for me to bother figuring out, which means this is
        // poorly designed
        notice.context = notice.context.or_else(|| self.context.clone());
        let endpoint = self.endpoint_uri();
        self.send_request(&endpoint, &notice)
    }

    /// This function returns a closure that can be passed to the `panic::set_hook`
    /// function. Only a single panic hook can be set at once, so exposing functionality
    /// this way forces you to manage your panic hooks yourself.
    pub fn panic_hook(&self) -> Box<dyn Fn(&PanicInfo<'_>) + Send + Sync + 'static> {
        let airbrake_client = self.clone();
        Box::new(move |panic_info: &PanicInfo<'_>| {
            let panic_backtrace = backtrace::Backtrace::new();
            let notice_error = NoticeError::from_panic_backtrace(panic_info, &panic_backtrace);
            let _ = airbrake_client
                .new_notice_builder()
                .add_notice(notice_error)
                .build()
                .send();
        })
    }
}

#[cfg(test)]
mod context_passthrough_tests {
    use crate::AirbrakeClient;
    use crate::ContextProperties;
    use serde_json::{self, Value};
    use std::str::FromStr;

    #[test]
    fn client_with_context_included_in_notices() {
        let client = AirbrakeClient::builder()
            .project_id("foo")
            .project_key("bar")
            .operating_system("SolarOS")
            .version("0.0.0")
            .severity("critical")
            .build()
            .unwrap();
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
        let client = AirbrakeClient::builder()
            .project_id("foo")
            .project_key("bar")
            .operating_system("SolarOS")
            .version("0.0.0")
            .component("controller")
            .build()
            .unwrap();
        let notice = client.new_notice_builder().severity("warning").build();

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
                "severity": "warning",
                "component": "controller"
            }
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(notice)
        );
    }
}

#[cfg(test)]
mod builder_tests {
    use super::AirbrakeClient;
    use super::AirbrakeClientBuilderError;

    #[test]
    fn endpoint_defaults_to_airbrake_server() {
        let client = AirbrakeClient::builder()
            .project_id("foo")
            .project_key("bar")
            .build();
        assert_eq!(
            "https://airbrake.io/api/v3/projects/foo/notices?key=bar",
            client.unwrap().endpoint_uri()
        );
    }

    #[test]
    fn project_sets_both_id_and_key() {
        let project_id = "foo";
        let project_key = "bar";
        let client1 = AirbrakeClient::builder()
            .project(project_id, project_key)
            .build();
        let client2 = AirbrakeClient::builder()
            .project_id(project_id)
            .project_key(project_key)
            .build();
        assert_eq!(
            "https://airbrake.io/api/v3/projects/foo/notices?key=bar",
            client1.unwrap().endpoint_uri()
        );
        assert_eq!(
            "https://airbrake.io/api/v3/projects/foo/notices?key=bar",
            client2.unwrap().endpoint_uri()
        );
    }

    #[test]
    fn config_build_fails_on_empty_project_id() {
        let client = AirbrakeClient::builder()
            .project_id("")
            .project_key("bar")
            .build();
        assert!(client.is_err());
        assert_eq!(
            client.unwrap_err(),
            AirbrakeClientBuilderError::EmptyProjectId
        )
    }

    #[test]
    fn client_build_fails_on_empty_project_key() {
        let client = AirbrakeClient::builder()
            .project_id("foo")
            .project_key("")
            .build();
        assert!(client.is_err());
        assert_eq!(
            client.unwrap_err(),
            AirbrakeClientBuilderError::EmptyProjectKey
        )
    }

    #[test]
    fn default_builder_fails_build() {
        let client = AirbrakeClient::builder().build();
        assert!(client.is_err());
        assert_eq!(
            client.unwrap_err(),
            AirbrakeClientBuilderError::MissingProjectId
        )
    }

    #[test]
    fn client_build_fails_on_missing_project_id() {
        let client = AirbrakeClient::builder().project_key("bar").build();
        assert!(client.is_err());
        assert_eq!(
            client.unwrap_err(),
            AirbrakeClientBuilderError::MissingProjectId
        )
    }

    #[test]
    fn client_build_fails_on_missing_project_key() {
        let client = AirbrakeClient::builder().project_id("foo").build();
        assert!(client.is_err());
        assert_eq!(
            client.unwrap_err(),
            AirbrakeClientBuilderError::MissingProjectKey
        )
    }
}
