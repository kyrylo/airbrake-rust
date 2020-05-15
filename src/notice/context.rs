
use super::{Notice, NoticeBuilder};

pub struct ContextBuilder {
    pub operating_system: Option<String>,
    pub hostname: Option<String>,
    pub language: Option<String>,
    pub environment: Option<String>,
    pub severity: Option<String>,
    pub version: Option<String>,
    pub url: Option<String>,
    pub root_directory: Option<String>,
    pub user: Option<ContextUser>,
    pub route: Option<String>,
    pub http_method: Option<String>,
}

impl ContextBuilder {
    pub fn new() -> ContextBuilder {
        ContextBuilder {
            operating_system: None,
            hostname: None,
            language: None,
            environment: None,
            severity: None,
            version: None,
            url: None,
            root_directory: None,
            user: None,
            route: None,
            http_method: None
        }
    }

    /// Set the operating system on the ContextBuilder
    pub fn operating_system(mut self, os: String) -> ContextBuilder {
        self.operating_system = Some(os);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn hostname(mut self, hostname: String) -> ContextBuilder {
        self.hostname = Some(hostname);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn language(mut self, language: String) -> ContextBuilder {
        self.language = Some(language);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn environment(mut self, environment: String) -> ContextBuilder {
        self.environment = Some(environment);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn severity(mut self, severity: String) -> ContextBuilder {
        self.severity = Some(severity);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn version(mut self, version: String) -> ContextBuilder {
        self.version = Some(version);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn url(mut self, url: String) -> ContextBuilder {
        self.url = Some(url);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn root_directory(mut self, root_directory: String) -> ContextBuilder {
        self.root_directory = Some(root_directory);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn user(mut self, user: ContextUser) -> ContextBuilder {
        self.user = Some(user);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn route(mut self, route: String) -> ContextBuilder {
        self.route = Some(route);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn http_method(mut self, http_method: String) -> ContextBuilder {
        self.http_method = Some(http_method);
        self
    }

    pub fn build(self) -> Context {
        Context {
            notifier: &CONTEXT_NOTIFIER,
            operating_system: self.operating_system,
            hostname: self.hostname,
            language: self.language,
            environment: self.environment,
            severity: self.severity,
            version: self.version,
            url: self.url,
            root_directory: self.root_directory,
            user: self.user,
            route: self.route,
            http_method: self.http_method
        }
    }

}

#[derive(Debug, Serialize, Clone)]
pub struct Context {
    // Builtin notifier
    pub notifier: &'static ContextNotifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename="os")]
    pub operating_system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename="rootDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ContextUser>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,

    #[serde(rename="httpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>
}

impl Context {
    pub fn builder() -> ContextBuilder {
        ContextBuilder::new()
    }

    pub fn new_notice_builder(&self) -> NoticeBuilder {
        Notice::builder().context(self.clone())
    }
}

/// This type is not intended to be used beyond the const CONTEXT_NOTIFIER
#[derive(Debug, Serialize, Clone)]
pub struct ContextNotifier {
    name: &'static str,
    version: &'static str,
    url: &'static str,
}

const NOTIFIER_NAME: &'static str = "airbrake-rust";
const NOTIFIER_URL: &'static str = "https://github.com/airbrake/airbrake-rust";
const NOTIFIER_VERSION: &'static str = "0.2.0";

pub const CONTEXT_NOTIFIER: ContextNotifier = ContextNotifier {
    name: NOTIFIER_NAME,
    version: NOTIFIER_VERSION,
    url: NOTIFIER_URL
};

#[derive(Debug, Serialize, Clone)]
pub struct ContextUser {
    id: Option<String>,
    name: Option<String>,
    email: Option<String>
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::collections::HashMap;
    use serde_json::{self, Value};
    use hyper::body::Body;
    use super::{Context, ContextUser};

    #[test]
    fn context_default_has_notifier() {
        let context = Context::builder().build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            }
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_operating_system() {
        let context = Context::builder()
            .operating_system("SolarOS".to_string())
            .build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "os": "SolarOS"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_hostname() {
        let context = Context::builder()
            .hostname("usw2.swa.foobar.com".to_string())
            .build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "hostname": "usw2.swa.foobar.com"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_language() {
        let context = Context::builder()
            .language("klingon".to_string())
            .build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "language": "klingon"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_environment() {
        let context = Context::builder()
            .environment("production".to_string())
            .build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "environment": "production"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_severity() {
        let context = Context::builder()
            .severity("critical".to_string())
            .build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "severity": "critical"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_version() {
        let context = Context::builder()
            .version("9000.0.1".to_string())
            .build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "version": "9000.0.1"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_url() {
        let context = Context::builder()
            .url("http://localhost/my/foobar".to_string())
            .build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "url": "http://localhost/my/foobar"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_root_directory() {
        let context = Context::builder()
            .root_directory("/dev/null".to_string())
            .build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "rootDirectory": "/dev/null"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_user() {
        let context_user = ContextUser {
            id: Some("foo".to_string()),
            email: Some("bar".to_string()),
            name: Some("baz".to_string())
        };
        let context = Context::builder()
            .user(context_user)
            .build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "user": {
                "id": "foo",
                "email": "bar",
                "name": "baz"
            }
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_route() {
        let context = Context::builder()
            .route("/foo/bar/baz".to_string())
            .build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "route": "/foo/bar/baz"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_http_method() {
        let context = Context::builder()
            .http_method("post".to_string())
            .build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "httpMethod": "post"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }
}

