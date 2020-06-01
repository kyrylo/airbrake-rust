use super::{Notice, NoticeBuilder};

#[derive(Debug, Clone, Default)]
pub struct ContextBuilder {
    _operating_system: Option<String>,
    _hostname: Option<String>,
    _language: Option<String>,
    _environment: Option<String>,
    _severity: Option<String>,
    _component: Option<String>,
    _action: Option<String>,
    _user_agent: Option<String>,
    _user_addr: Option<String>,
    _remote_addr: Option<String>,
    _version: Option<String>,
    _url: Option<String>,
    _root_directory: Option<String>,
    _user: Option<ContextUser>,
    _route: Option<String>,
    _http_method: Option<String>,
}

/// The methods for modifying the ContextBuilder properties are all delegated
/// to the default implementations in the ContextProperties trait. Though the
/// ContextProperty trait was intended for other structs, it can also be reused
/// here to reduce code duplication
impl ContextBuilder {
    pub fn new() -> ContextBuilder {
        ContextBuilder::default()
    }

    pub fn build(&self) -> Context {
        Context {
            notifier: &CONTEXT_NOTIFIER,
            operating_system: self._operating_system.clone(),
            hostname: self._hostname.clone(),
            language: self._language.clone(),
            environment: self._environment.clone(),
            severity: self._severity.clone(),
            component: self._component.clone(),
            action: self._action.clone(),
            user_agent: self._user_agent.clone(),
            user_addr: self._user_addr.clone(),
            remote_addr: self._remote_addr.clone(),
            version: self._version.clone(),
            url: self._url.clone(),
            root_directory: self._root_directory.clone(),
            user: self._user.clone(),
            route: self._route.clone(),
            http_method: self._http_method.clone(),
        }
    }
}

impl From<&Context> for ContextBuilder {
    fn from(context: &Context) -> ContextBuilder {
        ContextBuilder {
            _operating_system: context.operating_system.clone(),
            _hostname: context.hostname.clone(),
            _language: context.language.clone(),
            _environment: context.environment.clone(),
            _severity: context.severity.clone(),
            _component: context.component.clone(),
            _action: context.action.clone(),
            _user_agent: context.user_agent.clone(),
            _user_addr: context.user_addr.clone(),
            _remote_addr: context.remote_addr.clone(),
            _version: context.version.clone(),
            _url: context.url.clone(),
            _root_directory: context.root_directory.clone(),
            _user: context.user.clone(),
            _route: context.route.clone(),
            _http_method: context.http_method.clone(),
        }
    }
}

impl ContextProperties for ContextBuilder {
    fn get_context(&self) -> Option<ContextBuilder> {
        Some(self.clone())
    }

    fn set_context(&mut self, context: ContextBuilder) -> &mut Self {
        *self = context.clone();
        self
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Context {
    // Builtin notifier
    pub notifier: &'static ContextNotifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "os")]
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
    pub component: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    #[serde(rename = "userAgent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    #[serde(rename = "userAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_addr: Option<String>,

    #[serde(rename = "remoteAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_addr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "rootDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ContextUser>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,

    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
}

impl Context {
    pub fn builder() -> ContextBuilder {
        ContextBuilder::new()
    }

    pub fn new_notice_builder(&self) -> NoticeBuilder {
        let mut notice = Notice::builder();
        notice.context(&self.into());
        notice
    }
}

pub trait ContextProperties {
    fn get_context(&self) -> Option<ContextBuilder>;

    fn set_context(&mut self, context: ContextBuilder) -> &mut Self;

    fn operating_system(&mut self, os: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._operating_system = Some(os.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn hostname(&mut self, hostname: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._hostname = Some(hostname.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn language(&mut self, language: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._language = Some(language.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn environment(&mut self, environment: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._environment = Some(environment.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn severity(&mut self, severity: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._severity = Some(severity.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn component(&mut self, component: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._component = Some(component.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn action(&mut self, action: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._action = Some(action.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn user_agent(&mut self, user_agent: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._user_agent = Some(user_agent.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn user_addr(&mut self, user_addr: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._user_addr = Some(user_addr.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn remote_addr(&mut self, remote_addr: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._remote_addr = Some(remote_addr.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn version(&mut self, version: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._version = Some(version.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn url(&mut self, url: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._url = Some(url.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn root_directory(&mut self, root_directory: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._root_directory = Some(root_directory.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn user(&mut self, user: ContextUser) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._user = Some(user);
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn route(&mut self, route: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._route = Some(route.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }

    fn http_method(&mut self, http_method: &str) -> &mut Self {
        let updated_context = self
            .get_context()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c._http_method = Some(http_method.to_string());
                Some(c)
            })
            .unwrap();
        self.set_context(updated_context)
    }
}

/// This type is not intended to be used beyond the const CONTEXT_NOTIFIER
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ContextNotifier {
    name: &'static str,
    version: &'static str,
    url: &'static str,
}

const NOTIFIER_NAME: &str = "airbrake-rust";
const NOTIFIER_URL: &str = "https://github.com/airbrake/airbrake-rust";
const NOTIFIER_VERSION: &str = "0.2.0";

pub const CONTEXT_NOTIFIER: ContextNotifier = ContextNotifier {
    name: NOTIFIER_NAME,
    version: NOTIFIER_VERSION,
    url: NOTIFIER_URL,
};

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ContextUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

impl ContextUser {
    /// Get a blank ContextUser, whose values are all None
    pub fn empty() -> ContextUser {
        ContextUser {
            id: None,
            name: None,
            email: None,
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
    use super::ContextUser;
    use serde_json::{self, Value};
    use std::str::FromStr;

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
        let context = ContextUser::empty().id("foo").email("bar").name("baz");
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
    use super::{Context, ContextProperties, ContextUser};
    use serde_json::{self, Value};
    use std::str::FromStr;

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
        let context = Context::builder().operating_system("SolarOS").build();
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
        let context = Context::builder().hostname("usw2.swa.foobar.com").build();
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
        let context = Context::builder().language("klingon").build();
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
        let context = Context::builder().environment("production").build();
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
        let context = Context::builder().severity("critical").build();
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
    fn context_component() {
        let context = Context::builder().component("foobar").build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "component": "foobar"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_action() {
        let context = Context::builder().action("jump").build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "action": "jump"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_user_agent() {
        let context = Context::builder().user_agent("geko").build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "userAgent": "geko"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_user_addr() {
        let context = Context::builder().user_addr("0.0.0.0").build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "userAddr": "0.0.0.0"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_remote_addr() {
        let context = Context::builder().remote_addr("10.0.0.0").build();
        let expected_json = r#"
        {
            "notifier": {
                "name": "airbrake-rust",
                "version": "0.2.0",
                "url": "https://github.com/airbrake/airbrake-rust"
            },
            "remoteAddr": "10.0.0.0"
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            serde_json::json!(context)
        );
    }

    #[test]
    fn context_version() {
        let context = Context::builder().version("9000.0.1").build();
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
        let context = Context::builder().url("http://localhost/my/foobar").build();
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
        let context = Context::builder().root_directory("/dev/null").build();
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
        let context_user = ContextUser::empty().id("foo").email("bar").name("baz");
        let context = Context::builder().user(context_user).build();
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
        let context = Context::builder().route("/foo/bar/baz").build();
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
        let context = Context::builder().http_method("post").build();
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
