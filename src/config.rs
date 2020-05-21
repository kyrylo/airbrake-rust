
use std::env;
use crate::{Context, ContextBuilder, ContextUser};

const DEFAULT_HOSTNAME: &'static str = "https://airbrake.io";
const ENV_VAR_PROJECT_ID: &'static str = "AIRBRAKE_PROJECT_ID";
const ENV_VAR_PROJECT_KEY: &'static str = "AIRBRAKE_API_KEY";
const ENV_VAR_HOST: &'static str = "AIRBRAKE_HOST";

#[derive(Debug, PartialEq)]
pub enum AirbrakeConfigError {
    MissingProjectId,
    MissingProjectKey,
    EmptyProjectId,
    EmptyProjectKey
}

pub struct AirbrakeConfigBuilder {
    pub project_id: Option<String>,
    pub project_key: Option<String>,
    pub host: Option<String>,
    pub proxy: Option<String>,
    pub context: Option<ContextBuilder>,
}

impl AirbrakeConfigBuilder {
    pub fn new() -> AirbrakeConfigBuilder {
        AirbrakeConfigBuilder {
            project_id: None,
            project_key: None,
            host: None,
            proxy: None,
            context: None,
        }
    }

    pub fn configure<F>(&mut self, builder_callback: F) -> &mut AirbrakeConfigBuilder
    where F: Fn(&mut AirbrakeConfigBuilder)
    {
        builder_callback(self);
        self
    }

    pub fn project<'a>(&'a mut self, project_id: String, project_key: String) -> &'a mut AirbrakeConfigBuilder {
        self.project_id(project_id)
            .project_key(project_key)
    }

    pub fn project_id<'a>(&'a mut self, project_id: String) -> &'a mut AirbrakeConfigBuilder {
        self.project_id = Some(project_id);
        self
    }

    /// Builder can pull env vars from environment
    ///
    /// ```
    /// use std::env;
    /// use airbrake::AirbrakeConfig;
    ///
    /// // Set vars to set up our test
    /// env::set_var("AIRBRAKE_PROJECT_ID", "foo");
    /// env::set_var("AIRBRAKE_API_KEY", "bar");
    ///
    /// // Now build the config using just the environment variables
    /// let config = AirbrakeConfig::builder()
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
    /// use airbrake::AirbrakeConfig;
    ///
    /// // Set defaults to use if env vars are missing
    /// let default_project_id = "foo".to_owned();
    /// let default_project_key = "bar".to_owned();
    ///
    /// // Only set the project key using the env var
    /// env::set_var("AIRBRAKE_API_KEY", "baz");
    ///
    /// // Begin constructing a config builder
    /// let mut config = AirbrakeConfig::builder();
    /// if config.project_id_from_env().is_err() {
    ///     config.project_id(default_project_id);
    /// }
    /// if config.project_key_from_env().is_err() {
    ///     config.project_key(default_project_key);
    /// }
    /// let config = config.build().unwrap();
    /// assert_eq!(config.project_id, "foo");
    /// assert_eq!(config.project_key, "baz");
    /// ```
    ///
    pub fn project_id_from_env<'a>(&'a mut self) -> Result<&'a mut AirbrakeConfigBuilder, env::VarError> {
        match env::var(ENV_VAR_PROJECT_ID) {
            Ok(val) => {
                self.project_id = Some(val);
                Ok(self)
            },
            Err(e) => Err(e),
        }
    }

    pub fn project_key_from_env<'a>(&'a mut self) -> Result<&'a mut AirbrakeConfigBuilder, env::VarError> {
        match env::var(ENV_VAR_PROJECT_KEY) {
            Ok(val) => {
                self.project_key = Some(val);
                Ok(self)
            },
            Err(e) => Err(e),
        }
    }

    pub fn project_key<'a>(&'a mut self, project_key: String) -> &'a mut AirbrakeConfigBuilder {
        self.project_key = Some(project_key);
        self
    }

    pub fn host_from_env<'a>(&'a mut self) -> Result<&'a mut AirbrakeConfigBuilder, env::VarError> {
        match env::var(ENV_VAR_HOST) {
            Ok(val) => {
                self.host = Some(val);
                Ok(self)
            },
            Err(e) => Err(e),
        }
    }

    pub fn host<'a>(&'a mut self, host: String) -> &'a mut AirbrakeConfigBuilder {
        self.host = Some(host);
        self
    }

    pub fn proxy<'a>(&'a mut self, proxy: String) -> &'a mut AirbrakeConfigBuilder {
        self.proxy = Some(proxy);
        self
    }

    // Sets the configurations context to an existing context builder
    pub fn context<'a>(&'a mut self, context: ContextBuilder) -> &'a mut AirbrakeConfigBuilder {
        self.context = Some(context);
        self
    }

    /// Set the operating_system on the configurations context
    pub fn operating_system<'a>(&'a mut self, os: String) -> &'a mut AirbrakeConfigBuilder {
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
    pub fn hostname<'a>(&'a mut self, hostname: String) -> &'a mut AirbrakeConfigBuilder {
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
    pub fn language<'a>(&'a mut self, language: String) -> &'a mut AirbrakeConfigBuilder {
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
    pub fn environment<'a>(&'a mut self, environment: String) -> &'a mut AirbrakeConfigBuilder {
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
    pub fn severity<'a>(&'a mut self, severity: String) -> &'a mut AirbrakeConfigBuilder {
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
    pub fn version<'a>(&'a mut self, version: String) -> &'a mut AirbrakeConfigBuilder {
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
    pub fn url<'a>(&'a mut self, url: String) -> &'a mut AirbrakeConfigBuilder {
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
    pub fn root_directory<'a>(&'a mut self, root_directory: String) -> &'a mut AirbrakeConfigBuilder {
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
    pub fn user<'a>(&'a mut self, user: ContextUser) -> &'a mut AirbrakeConfigBuilder {
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
    pub fn route<'a>(&'a mut self, route: String) -> &'a mut AirbrakeConfigBuilder {
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
    pub fn http_method<'a>(&'a mut self, http_method: String) -> &'a mut AirbrakeConfigBuilder {
        self.context = self.context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.http_method(http_method);
                Some(c)
            });
        self
    }

    pub fn build(&self) -> Result<AirbrakeConfig, AirbrakeConfigError> {
        let project_id = match &self.project_id {
            Some( id ) => id,
            None => return Err( AirbrakeConfigError::MissingProjectId )
        };
        let project_key = match &self.project_key {
            Some( key ) => key,
            None => return Err( AirbrakeConfigError::MissingProjectKey )
        };
        if project_id.is_empty() {
            return Err( AirbrakeConfigError::EmptyProjectId )
        }
        if project_key.is_empty() {
            return Err( AirbrakeConfigError::EmptyProjectKey )
        }
        let context = self.context.clone().and_then(|c| Some(c.build()));

        Ok(AirbrakeConfig {
            project_id: project_id.to_string(),
            project_key: project_key.to_string(),
            host: self.host.clone().unwrap_or(DEFAULT_HOSTNAME.to_owned()),
            proxy: self.proxy.clone(),
            context: context
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AirbrakeConfig {
    pub project_id: String,
    pub project_key: String,
    pub host: String,
    pub proxy: Option<String>,
    pub context: Option<Context>
}

impl AirbrakeConfig {
    /// Enables easy creation of the config object
    ///
    /// ```
    /// use airbrake::AirbrakeConfig;
    ///
    /// let my_project_id = "foo".to_owned();
    /// let my_project_key = "bar".to_owned();
    /// let config = AirbrakeConfig::builder()
    ///     .project(my_project_id, my_project_key)
    ///     .build();
    /// ```
    pub fn builder() -> AirbrakeConfigBuilder {
        AirbrakeConfigBuilder::new()
    }

    pub fn new(project_id: String, project_key: String) -> Result<AirbrakeConfig, AirbrakeConfigError> {
        AirbrakeConfig::builder()
            .project_id(project_id)
            .project_key(project_key)
            .build()
    }

    pub fn endpoint_uri(&self) -> String {
        format!(
            "{}/api/v3/projects/{}/notices?key={}",
            self.host,
            self.project_id,
            self.project_key,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::AirbrakeConfig;
    use super::AirbrakeConfigError;

    #[test]
    fn endpoint_defaults_to_airbrake_server() {
        let project_id = "foo".to_owned();
        let project_key = "bar".to_owned();
        let config = AirbrakeConfig::builder()
            .project_id(project_id)
            .project_key(project_key)
            .build();
        assert_eq!(
            "https://airbrake.io/api/v3/projects/foo/notices?key=bar",
            config.unwrap().endpoint_uri()
        );
    }

    #[test]
    fn project_sets_both_id_and_key() {
        let project_id = "foo".to_owned();
        let project_key = "bar".to_owned();
        let config1 = AirbrakeConfig::builder()
            .project(project_id.clone(), project_key.clone())
            .build();
        let config2 = AirbrakeConfig::builder()
            .project_id(project_id)
            .project_key(project_key)
            .build();
        assert_eq!(config1, config2);
        assert_eq!(
            "https://airbrake.io/api/v3/projects/foo/notices?key=bar",
            config1.unwrap().endpoint_uri()
        )
    }

    #[test]
    fn config_build_fails_on_empty_project_id() {
        let config = AirbrakeConfig::builder()
            .project_id("".to_owned())
            .project_key("bar".to_owned())
            .build();
        assert_eq!(config, Err(AirbrakeConfigError::EmptyProjectId))
    }

    #[test]
    fn config_build_fails_on_empty_project_key() {
        let config = AirbrakeConfig::builder()
            .project_id("foo".to_owned())
            .project_key("".to_owned())
            .build();
        assert_eq!(config, Err(AirbrakeConfigError::EmptyProjectKey))
    }

    #[test]
    fn default_builder_fails_build() {
        let config = AirbrakeConfig::builder().build();
        assert_eq!(config, Err(AirbrakeConfigError::MissingProjectId))
    }

    #[test]
    fn config_build_fails_on_missing_project_id() {
        let config = AirbrakeConfig::builder()
            .project_key("bar".to_owned())
            .build();
        assert_eq!(config, Err(AirbrakeConfigError::MissingProjectId))
    }

    #[test]
    fn config_build_fails_on_missing_project_key() {
        let config = AirbrakeConfig::builder()
            .project_id("foo".to_owned())
            .build();
        assert_eq!(config, Err(AirbrakeConfigError::MissingProjectKey))
    }
}
