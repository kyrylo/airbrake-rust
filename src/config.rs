#[derive(Debug, Clone)]
pub struct Config {
    pub project_id: String,
    pub project_key: String,
    pub host: String,
    pub workers: i32,
    pub proxy: String,
    pub app_version: String,
    pub environment: Option<String>,
    pub component: Option<String>,
    pub os: Option<String>,
    pub hostname: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            project_id: "0".to_owned(),
            project_key: "0".to_owned(),
            host: "https://airbrake.io".to_owned(),
            workers: 1,
            proxy: String::new(),
            app_version: String::new(),
            environment: None,
            component: None,
            os: None,
            hostname: None,
        }
    }

    pub fn endpoint(&self) -> String {
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
