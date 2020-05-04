
use std::env;
use hyper::Uri;

const DEFAULT_HOSTNAME: &'static str = "https://airbrake.io";
const ENV_VAR_PROJECT_ID: &'static str = "AIRBRAKE_PROJECT_ID";
const ENV_VAR_PROJECT_KEY: &'static str = "AIRBRAKE_API_KEY";
const ENV_VAR_HOST: &'static str = "AIRBRAKE_HOST";

pub struct ConfigBuilder {
    pub project_id: Option<String>,
    pub project_key: Option<String>,
    pub host: Option<String>,
    pub proxy: Option<String>
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            project_id: None,
            project_key: None,
            host: None,
            proxy: None
        }
    }

    pub fn project<'a>(&'a mut self, project_id: String, project_key: String) -> &'a mut ConfigBuilder {
        self.project_id(project_id)
            .project_key(project_key)
    }

    pub fn project_id<'a>(&'a mut self, project_id: String) -> &'a mut ConfigBuilder {
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
    /// let config = config.build();
    /// assert_eq!(config.project_id, "foo");
    /// assert_eq!(config.project_key, "baz");
    /// ```
    ///
    pub fn project_id_from_env<'a>(&'a mut self) -> Result<&'a mut ConfigBuilder, env::VarError> {
        match env::var(ENV_VAR_PROJECT_ID) {
            Ok(val) => {
                self.project_id = Some(val);
                Ok(self)
            },
            Err(e) => Err(e),
        }
    }

    pub fn project_key_from_env<'a>(&'a mut self) -> Result<&'a mut ConfigBuilder, env::VarError> {
        match env::var(ENV_VAR_PROJECT_KEY) {
            Ok(val) => {
                self.project_key = Some(val);
                Ok(self)
            },
            Err(e) => Err(e),
        }
    }

    pub fn project_key<'a>(&'a mut self, project_key: String) -> &'a mut ConfigBuilder {
        self.project_key = Some(project_key);
        self
    }

    pub fn host_from_env<'a>(&'a mut self) -> Result<&'a mut ConfigBuilder, env::VarError> {
        match env::var(ENV_VAR_HOST) {
            Ok(val) => {
                self.host = Some(val);
                Ok(self)
            },
            Err(e) => Err(e),
        }
    }

    pub fn host<'a>(&'a mut self, host: String) -> &'a mut ConfigBuilder {
        self.host = Some(host);
        self
    }

    pub fn proxy<'a>(&'a mut self, proxy: String) -> &'a mut ConfigBuilder {
        self.proxy = Some(proxy);
        self
    }

    pub fn build(&self) -> AirbrakeConfig {
        AirbrakeConfig {
            project_id: self.project_id.clone().unwrap(),
            project_key: self.project_key.clone().unwrap(),
            host: self.host.clone().unwrap_or(DEFAULT_HOSTNAME.to_owned()),
            proxy: self.proxy.clone()
        }
    }
}

#[derive(Debug, Clone)]
pub struct AirbrakeConfig {
    pub project_id: String,
    pub project_key: String,
    pub host: String,
    pub proxy: Option<String>
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
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
    pub fn new(project_id: String, project_key: String) -> AirbrakeConfig {
        AirbrakeConfig::builder()
            .project_id(project_id)
            .project_key(project_key)
            .build()
    }

    pub fn endpoint(&self) -> String {
        format!(
            "{}/api/v3/projects/{}/notices?key={}",
            self.host,
            self.project_id,
            self.project_key,
        )
    }

    pub fn endpoint_uri(&self) -> Uri {
        self.endpoint().parse().expect("malformed URL")
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn endpoint_defaults_to_airbrake_server() {
        assert_eq!(
            "https://airbrake.io/api/v3/projects/0/notices?key=0",
            Config::new().endpoint()
        );
    }

    #[test]
    fn project_id_modifies_endpoint() {
        let mut config = Config::new();
        config.project_id = "123".to_owned();

        assert_eq!(
            "https://airbrake.io/api/v3/projects/123/notices?key=0",
            config.endpoint()
        );
    }

    #[test]
    fn project_key_modifies_endpoint() {
        let mut config = Config::new();
        config.project_key = "bingo".to_owned();

        assert_eq!(
            "https://airbrake.io/api/v3/projects/0/notices?key=bingo",
            config.endpoint()
        );
    }

    #[test]
    fn host_modifies_endpoint() {
        let mut config = Config::new();
        config.host = "http://localhost:9090".to_owned();

        assert_eq!(
            "http://localhost:9090/api/v3/projects/0/notices?key=0",
            config.endpoint()
        );
    }
}
