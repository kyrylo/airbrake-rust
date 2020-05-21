
use super::{Notice, NoticeBuilder};

#[derive(Debug, Clone)]
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
    pub fn operating_system<'a>(&'a mut self, os: &str) -> &'a mut ContextBuilder {
        self.operating_system = Some(os.to_string());
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn hostname<'a>(&'a mut self, hostname: &str) -> &'a mut ContextBuilder {
        self.hostname = Some(hostname.to_string());
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn language<'a>(&'a mut self, language: &str) -> &'a mut ContextBuilder {
        self.language = Some(language.to_string());
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn environment<'a>(&'a mut self, environment: &str) -> &'a mut ContextBuilder {
        self.environment = Some(environment.to_string());
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn severity<'a>(&'a mut self, severity: &str) -> &'a mut ContextBuilder {
        self.severity = Some(severity.to_string());
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn version<'a>(&'a mut self, version: &str) -> &'a mut ContextBuilder {
        self.version = Some(version.to_string());
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn url<'a>(&'a mut self, url: &str) -> &'a mut ContextBuilder {
        self.url = Some(url.to_string());
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn root_directory<'a>(&'a mut self, root_directory: &str) -> &'a mut ContextBuilder {
        self.root_directory = Some(root_directory.to_string());
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn user<'a>(&'a mut self, user: ContextUser) -> &'a mut ContextBuilder {
        self.user = Some(user);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn route<'a>(&'a mut self, route: &str) -> &'a mut ContextBuilder {
        self.route = Some(route.to_string());
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn http_method<'a>(&'a mut self, http_method: &str) -> &'a mut ContextBuilder {
        self.http_method = Some(http_method.to_string());
        self
    }

    pub fn build(&self) -> Context {
        Context {
            notifier: &CONTEXT_NOTIFIER,
            operating_system: self.operating_system.clone(),
            hostname: self.hostname.clone(),
            language: self.language.clone(),
            environment: self.environment.clone(),
            severity: self.severity.clone(),
            version: self.version.clone(),
            url: self.url.clone(),
            root_directory: self.root_directory.clone(),
            user: self.user.clone(),
            route: self.route.clone(),
            http_method: self.http_method.clone()
        }
    }

}

impl From<&Context> for ContextBuilder {
    fn from(context: &Context) -> ContextBuilder {
        ContextBuilder {
            operating_system: context.operating_system.clone(),
            hostname: context.hostname.clone(),
            language: context.language.clone(),
            environment: context.environment.clone(),
            severity: context.severity.clone(),
            version: context.version.clone(),
            url: context.url.clone(),
            root_directory: context.root_directory.clone(),
            user: context.user.clone(),
            route: context.route.clone(),
            http_method: context.http_method.clone()
        }
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
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
        Notice::builder().context(self.into())
    }
}

/// This type is not intended to be used beyond the const CONTEXT_NOTIFIER
#[derive(Debug, Serialize, Clone, PartialEq)]
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

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ContextUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>
}

impl ContextUser {
    /// Get a blank ContextUser, whose values are all None
    pub fn empty() -> ContextUser {
        ContextUser {
            id: None,
            name: None,
            email: None
        }
    }

    /// Set the id on the ContextUser
    pub fn id(mut self, id: &str) -> ContextUser {
        self.id = Some(id.to_string());
        self
    }

    /// Set the name on the ContextUser
    pub fn name(mut self, name: &str) -> ContextUser {
        self.name = Some(name.to_string());
        self
    }

    /// Set the email on the ContextUser
    pub fn email(mut self, email: &str) -> ContextUser {
        self.email = Some(email.to_string());
        self
    }
}

#[cfg(test)]
mod context_user_tests {
    use std::str::FromStr;
    use serde_json::{self, Value};
    use super::ContextUser;

    #[test]
    fn context_user_default() {
        let context = ContextUser::empty();
        let expected_json = r#"
        {}
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_user_with_id() {
        let context = ContextUser::empty().id("foo");
        let expected_json = r#"
        {
            "id": "foo"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_user_with_name() {
        let context = ContextUser::empty().name("foo");
        let expected_json = r#"
        {
            "name": "foo"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_user_with_email() {
        let context = ContextUser::empty().email("foo");
        let expected_json = r#"
        {
            "email": "foo"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_user_with_all_fields() {
        let context = ContextUser::empty()
            .id("foo")
            .email("bar")
            .name("baz");
        let expected_json = r#"
        {
            "id": "foo",
            "email": "bar",
            "name": "baz"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }
}

#[cfg(test)]
mod context_tests {
    use std::str::FromStr;
    use serde_json::{self, Value};
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
            .operating_system("SolarOS")
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
            .hostname("usw2.swa.foobar.com")
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
            .language("klingon")
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
            .environment("production")
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
            .severity("critical")
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
            .version("9000.0.1")
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
            .url("http://localhost/my/foobar")
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
            .root_directory("/dev/null")
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
        let context_user = ContextUser::empty()
            .id("foo")
            .email("bar")
            .name("baz");
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
            .route("/foo/bar/baz")
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
            .http_method("post")
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

