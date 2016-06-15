#[derive(Debug)]
pub struct Config {
    pub project_id: String,
    pub project_key: String,
    pub host: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            project_id: "0".to_owned(),
            project_key: "0".to_owned(),
            host: "https://airbrake.io".to_owned(),
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
