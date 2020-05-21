
use std::env;
use std::panic::PanicInfo;
use std::marker::{Send, Sync};
use std::time::Instant;
use log::warn;
use reqwest::blocking::Client;
use serde::Serialize;

use crate::Notice;
use crate::NoticeError;
use crate::NoticeBuilder;
use crate::{Context, ContextBuilder, ContextUser};

const DEFAULT_HOSTNAME: &'static str = "https://airbrake.io";
const ENV_VAR_PROJECT_ID: &'static str = "AIRBRAKE_PROJECT_ID";
const ENV_VAR_PROJECT_KEY: &'static str = "AIRBRAKE_API_KEY";
const ENV_VAR_HOST: &'static str = "AIRBRAKE_HOST";

#[derive(Debug, PartialEq)]
pub enum AirbrakeClientBuilderError {
    MissingProjectId,
    MissingProjectKey,
    EmptyProjectId,
    EmptyProjectKey
}

pub struct AirbrakeClientBuilder {
    pub project_id: Option<String>,
    pub project_key: Option<String>,
    pub host: Option<String>,
    pub proxy: Option<String>,
    pub context: Option<ContextBuilder>,
}

impl AirbrakeClientBuilder {
    pub fn new() -> AirbrakeClientBuilder {
        AirbrakeClientBuilder {
            project_id: None,
            project_key: None,
            host: None,
            proxy: None,
            context: None,
        }
    }

    pub fn configure<F>(&mut self, builder_callback: F) -> &mut AirbrakeClientBuilder
    where F: Fn(&mut AirbrakeClientBuilder)
    {
        builder_callback(self);
        self
    }

    pub fn project<'a>(&'a mut self, project_id: &str, project_key: &str) -> &'a mut AirbrakeClientBuilder {
        self.project_id(project_id)
            .project_key(project_key)
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
    pub fn project_id_from_env<'a>(&'a mut self) -> Result<&'a mut AirbrakeClientBuilder, env::VarError> {
        match env::var(ENV_VAR_PROJECT_ID) {
            Ok(val) => {
                self.project_id = Some(val);
                Ok(self)
            },
            Err(e) => Err(e),
        }
    }

    pub fn project_key_from_env<'a>(&'a mut self) -> Result<&'a mut AirbrakeClientBuilder, env::VarError> {
        match env::var(ENV_VAR_PROJECT_KEY) {
            Ok(val) => {
                self.project_key = Some(val);
                Ok(self)
            },
            Err(e) => Err(e),
        }
    }

    pub fn project_key<'a>(&'a mut self, project_key: &str) -> &'a mut AirbrakeClientBuilder {
        self.project_key = Some(project_key.to_string());
        self
    }

    pub fn host_from_env<'a>(&'a mut self) -> Result<&'a mut AirbrakeClientBuilder, env::VarError> {
        match env::var(ENV_VAR_HOST) {
            Ok(val) => {
                self.host = Some(val);
                Ok(self)
            },
            Err(e) => Err(e),
        }
    }

    pub fn host<'a>(&'a mut self, host: &str) -> &'a mut AirbrakeClientBuilder {
        self.host = Some(host.to_string());
        self
    }

    pub fn proxy<'a>(&'a mut self, proxy: &str) -> &'a mut AirbrakeClientBuilder {
        self.proxy = Some(proxy.to_string());
        self
    }

    // Sets the configurations context to an existing context builder
    pub fn context<'a>(&'a mut self, context: ContextBuilder) -> &'a mut AirbrakeClientBuilder {
        self.context = Some(context);
        self
    }

    /// Set the operating_system on the configurations context
    pub fn operating_system<'a>(&'a mut self, os: &str) -> &'a mut AirbrakeClientBuilder {
        self.context = self.context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.operating_system(os);
                Some(c)
            });
        self
    }

    /// Set the hostname on the configurations context
    pub fn hostname<'a>(&'a mut self, hostname: &str) -> &'a mut AirbrakeClientBuilder {
        self.context = self.context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.hostname(hostname);
                Some(c)
            });
        self
    }

    /// Set the language on the configurations context
    pub fn language<'a>(&'a mut self, language: &str) -> &'a mut AirbrakeClientBuilder {
        self.context = self.context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.language(language);
                Some(c)
            });
        self
    }

    /// Set the environment on the configurations context
    pub fn environment<'a>(&'a mut self, environment: &str) -> &'a mut AirbrakeClientBuilder {
        self.context = self.context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.environment(environment);
                Some(c)
            });
        self
    }

    /// Set the severity on the configurations context
    pub fn severity<'a>(&'a mut self, severity: &str) -> &'a mut AirbrakeClientBuilder {
        self.context = self.context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.severity(severity);
                Some(c)
            });
        self
    }

    /// Set the version on the configurations context
    pub fn version<'a>(&'a mut self, version: &str) -> &'a mut AirbrakeClientBuilder {
        self.context = self.context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.version(version);
                Some(c)
            });
        self
    }

    /// Set the url on the configurations context
    pub fn url<'a>(&'a mut self, url: &str) -> &'a mut AirbrakeClientBuilder {
        self.context = self.context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.url(url);
                Some(c)
            });
        self
    }

    /// Set the root_directory on the configurations context
    pub fn root_directory<'a>(&'a mut self, root_directory: &str) -> &'a mut AirbrakeClientBuilder {
        self.context = self.context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.root_directory(root_directory);
                Some(c)
            });
        self
    }

    /// Set the user on the configurations context
    pub fn user<'a>(&'a mut self, user: ContextUser) -> &'a mut AirbrakeClientBuilder {
        self.context = self.context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.user(user);
                Some(c)
            });
        self
    }

    /// Set the route on the configurations context
    pub fn route<'a>(&'a mut self, route: &str) -> &'a mut AirbrakeClientBuilder {
        self.context = self.context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.route(route);
                Some(c)
            });
        self
    }

    /// Set the http_method on the configurations context
    pub fn http_method<'a>(&'a mut self, http_method: &str) -> &'a mut AirbrakeClientBuilder {
        self.context = self.context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.http_method(http_method);
                Some(c)
            });
        self
    }

    pub fn build(&self) -> Result<AirbrakeClient, AirbrakeClientBuilderError> {
        let project_id = match &self.project_id {
            Some( id ) => id,
            None => return Err( AirbrakeClientBuilderError::MissingProjectId )
        };
        let project_key = match &self.project_key {
            Some( key ) => key,
            None => return Err( AirbrakeClientBuilderError::MissingProjectKey )
        };
        if project_id.is_empty() {
            return Err( AirbrakeClientBuilderError::EmptyProjectId )
        }
        if project_key.is_empty() {
            return Err( AirbrakeClientBuilderError::EmptyProjectKey )
        }
        let context = self.context.clone().and_then(|c| Some(c.build()));

        Ok(AirbrakeClient {
            client: Client::new(),
            project_id: project_id.to_string(),
            project_key: project_key.to_string(),
            host: self.host.clone().unwrap_or(DEFAULT_HOSTNAME.to_owned()),
            proxy: self.proxy.clone(),
            context: context
        })
    }
}

#[derive(Debug, Clone)]
pub struct AirbrakeClient {
    client: Client,

    project_id: String,
    project_key: String,
    host: String,
    proxy: Option<String>,
    context: Option<Context>
}

impl AirbrakeClient {
    // pub fn new(config: AirbrakeConfig) -> AirbrakeClient {
    //     AirbrakeClient {
    //         client: Client::new(),
    //         config: config
    //     }
    // }

    pub fn builder() -> AirbrakeClientBuilder {
        AirbrakeClientBuilder::new()
    }

    fn endpoint_uri(&self) -> String {
        format!(
            "{}/api/v3/projects/{}/notices?key={}",
            self.host,
            self.project_id,
            self.project_key,
        )
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
        let notice_builder = match &self.context {
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
        notice.context = notice.context.or_else(|| {self.context.clone()});
        let endpoint = self.endpoint_uri();
        self.send_request(&endpoint, &notice)
    }

    /// This function returns a closure that can be passed to the `panic::set_hook`
    /// function. Only a single panic hook can be set at once, so exposing functinality
    /// this way forces you to manage your panic hooks yourself.
    pub fn panic_hook(self) -> Box<dyn Fn(&PanicInfo<'_>) + Send + Sync + 'static> {
        let airbrake_client = self.clone();
        Box::new(move |panic_info: &PanicInfo<'_>| {
            let panic_backtrace = backtrace::Backtrace::new();
            let notice_error = NoticeError::from_panic_backtrace(panic_info, &panic_backtrace);
            airbrake_client.new_notice_builder()
                .add_notice(notice_error)
                .build()
                .send();
        })
    }
}

#[cfg(test)]
mod context_passthrough_tests {
    use std::str::FromStr;
    use serde_json::{self, Value};
    use crate::AirbrakeClient;

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
            .build()
            .unwrap();
        let notice = client.new_notice_builder()
            .severity("warning")
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
        assert_eq!(client.unwrap_err(), AirbrakeClientBuilderError::EmptyProjectId)
    }

    #[test]
    fn client_build_fails_on_empty_project_key() {
        let client = AirbrakeClient::builder()
            .project_id("foo")
            .project_key("")
            .build();
        assert!(client.is_err());
        assert_eq!(client.unwrap_err(), AirbrakeClientBuilderError::EmptyProjectKey)
    }

    #[test]
    fn default_builder_fails_build() {
        let client = AirbrakeClient::builder().build();
        assert!(client.is_err());
        assert_eq!(client.unwrap_err(), AirbrakeClientBuilderError::MissingProjectId)
    }

    #[test]
    fn client_build_fails_on_missing_project_id() {
        let client = AirbrakeClient::builder()
            .project_key("bar")
            .build();
        assert!(client.is_err());
        assert_eq!(client.unwrap_err(), AirbrakeClientBuilderError::MissingProjectId)
    }

    #[test]
    fn client_build_fails_on_missing_project_key() {
        let client = AirbrakeClient::builder()
            .project_id("foo")
            .build();
        assert!(client.is_err());
        assert_eq!(client.unwrap_err(), AirbrakeClientBuilderError::MissingProjectKey)
    }
}
