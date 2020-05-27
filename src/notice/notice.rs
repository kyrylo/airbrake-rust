use serde_json::{self, Value};

use super::{NoticeError, NoticeTrace};
use crate::{
    backtrace::Backtrace, AirbrakeClient, AirbrakeClientError, Context, ContextBuilder, ContextUser,
};
use log::debug;
use std::collections::HashMap;
use std::error::Error;
use std::string::ToString;

#[derive(Default)]
pub struct NoticeBuilder<'a> {
    pub client: Option<&'a AirbrakeClient>,
    pub errors: Vec<NoticeError>,
    pub context: Option<ContextBuilder>,
    pub environment: Option<HashMap<String, String>>,
    pub session: Option<HashMap<String, String>>,
    pub params: Option<HashMap<String, String>>,
}

impl<'a> NoticeBuilder<'a> {
    /// Set the environment on the NoticeBuilder
    pub fn new() -> NoticeBuilder<'a> {
        NoticeBuilder::default()
    }

    pub fn set_client(mut self, client: &'a AirbrakeClient) -> NoticeBuilder<'a> {
        self.client = Some(client);
        self
    }

    /// Add multiple NoticeErrors from an iterator
    pub fn add_notices<T: Iterator<Item = NoticeError>>(
        mut self,
        notice_errors: T,
    ) -> NoticeBuilder<'a> {
        self.errors.extend(notice_errors);
        self
    }

    /// Add a single NoticeError
    pub fn add_notice(mut self, notice_error: NoticeError) -> NoticeBuilder<'a> {
        self.errors.push(notice_error);
        self
    }

    /// Add multiple Errors from an iterator
    pub fn add_errors<T: Iterator<Item = E>, E: Error>(self, errors: T) -> NoticeBuilder<'a> {
        let notice_errors = errors.map(|x| x.into());
        self.add_notices(notice_errors)
    }

    /// Add a single Error
    pub fn add_error<E: Error>(self, error: E) -> NoticeBuilder<'a> {
        let notice_error = NoticeError::from(error);
        self.add_notice(notice_error)
    }

    pub fn add_error_with_backtrace<E: Error>(
        self,
        error: E,
        backtrace: Backtrace,
    ) -> NoticeBuilder<'a> {
        let mut notice_error = NoticeError::from(error);
        notice_error.backtrace = Some(NoticeTrace::from(&backtrace));
        self.add_notice(notice_error)
    }

    /// Set the context on the NoticeBuilder
    pub fn context(mut self, context: ContextBuilder) -> NoticeBuilder<'a> {
        self.context = Some(context);
        self
    }

    /// Set the operating_system on the configurations context
    pub fn operating_system(mut self, os: &str) -> NoticeBuilder<'a> {
        self.context = self
            .context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.operating_system(os);
                Some(c)
            });
        self
    }

    /// Set the hostname on the configurations context
    pub fn hostname(mut self, hostname: &str) -> NoticeBuilder<'a> {
        self.context = self
            .context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.hostname(hostname);
                Some(c)
            });
        self
    }

    /// Set the language on the configurations context
    pub fn language(mut self, language: &str) -> NoticeBuilder<'a> {
        self.context = self
            .context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.language(language);
                Some(c)
            });
        self
    }

    /// Set the environment on the configurations context
    pub fn context_environment(mut self, environment: &str) -> NoticeBuilder<'a> {
        self.context = self
            .context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.environment(environment);
                Some(c)
            });
        self
    }

    /// Set the severity on the configurations context
    pub fn severity(mut self, severity: &str) -> NoticeBuilder<'a> {
        self.context = self
            .context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.severity(severity);
                Some(c)
            });
        self
    }

    /// Set the version on the configurations context
    pub fn version(mut self, version: &str) -> NoticeBuilder<'a> {
        self.context = self
            .context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.version(version);
                Some(c)
            });
        self
    }

    /// Set the url on the configurations context
    pub fn url(mut self, url: &str) -> NoticeBuilder<'a> {
        self.context = self
            .context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.url(url);
                Some(c)
            });
        self
    }

    /// Set the root_directory on the configurations context
    pub fn root_directory(mut self, root_directory: &str) -> NoticeBuilder<'a> {
        self.context = self
            .context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.root_directory(root_directory);
                Some(c)
            });
        self
    }

    /// Set the user on the configurations context
    pub fn user(mut self, user: ContextUser) -> NoticeBuilder<'a> {
        self.context = self
            .context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.user(user);
                Some(c)
            });
        self
    }

    /// Set the route on the configurations context
    pub fn route(mut self, route: &str) -> NoticeBuilder<'a> {
        self.context = self
            .context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.route(route);
                Some(c)
            });
        self
    }

    /// Set the http_method on the configurations context
    pub fn http_method(mut self, http_method: &str) -> NoticeBuilder<'a> {
        self.context = self
            .context
            .clone()
            .or_else(|| Some(Context::builder()))
            .and_then(|mut c| {
                c.http_method(http_method);
                Some(c)
            });
        self
    }

    /// Set the environment on the NoticeBuilder
    pub fn environment(mut self, environment: HashMap<String, String>) -> NoticeBuilder<'a> {
        self.environment = Some(environment);
        self
    }

    /// Add environment to the NoticeBuilder
    /// ```
    /// use airbrake::Notice;
    /// let notice = Notice::builder()
    ///     .add_environment("PORT", "443")
    ///     .add_environment("CODE_NAME", "gorilla")
    ///     .build();
    /// ```
    pub fn add_environment(mut self, key: &str, value: &str) -> NoticeBuilder<'a> {
        self.environment = self
            .environment
            .or_else(|| Some(HashMap::new()))
            .and_then(|mut h| {
                h.insert(key.to_string(), value.to_string());
                Some(h)
            });
        self
    }

    /// Set the environment on the NoticeBuilder
    pub fn session(mut self, session: HashMap<String, String>) -> NoticeBuilder<'a> {
        self.session = Some(session);
        self
    }

    /// Add session to the NoticeBuilder
    /// ```
    /// use airbrake::Notice;
    /// let notice = Notice::builder()
    ///     .add_session("basketId", "123")
    ///     .add_session("userId", "456")
    ///     .build();
    /// ```
    pub fn add_session(mut self, key: &str, value: &str) -> NoticeBuilder<'a> {
        self.session = self
            .session
            .or_else(|| Some(HashMap::new()))
            .and_then(|mut h| {
                h.insert(key.to_string(), value.to_string());
                Some(h)
            });
        self
    }

    /// Set the environment on the NoticeBuilder
    pub fn params(mut self, params: HashMap<String, String>) -> NoticeBuilder<'a> {
        self.params = Some(params);
        self
    }

    /// Add param to the NoticeBuilder
    /// ```
    /// use airbrake::Notice;
    /// let notice = Notice::builder()
    ///     .add_param("page", "3")
    ///     .add_param("sort", "name")
    ///     .add_param("direction", "asc")
    ///     .build();
    /// ```
    pub fn add_param(mut self, key: &str, value: &str) -> NoticeBuilder<'a> {
        self.params = self
            .params
            .or_else(|| Some(HashMap::new()))
            .and_then(|mut h| {
                h.insert(key.to_string(), value.to_string());
                Some(h)
            });
        self
    }

    /// Executes the command as a child process, which is returned.
    pub fn build(self) -> Notice<'a> {
        let context = self.context.clone().map(|c| c.build());
        Notice {
            client: self.client,
            errors: self.errors,
            context,
            environment: self.environment,
            session: self.session,
            params: self.params,
        }
    }
}

/// NoticeBuilder can be produced from a Context
///
/// The context specified in a Notice won't change often, and will typically
/// already exist while creating a new Notice so it only makes sense to
/// begin the Notice construction based on the context.
/// ```
/// use airbrake::{Context, NoticeBuilder};
///
/// let context = Context::builder();
/// let notice_builder = NoticeBuilder::from(&context);
/// ```
impl<'a> From<&ContextBuilder> for NoticeBuilder<'a> {
    fn from(context: &ContextBuilder) -> NoticeBuilder<'a> {
        NoticeBuilder::new().context(context.clone())
    }
}

impl<'a, E: Error> From<E> for NoticeBuilder<'a> {
    fn from(error: E) -> NoticeBuilder<'a> {
        NoticeBuilder::new().add_error(error)
    }
}

#[derive(Debug, Serialize)]
pub struct Notice<'a> {
    #[serde(skip)]
    pub client: Option<&'a AirbrakeClient>,

    pub errors: Vec<NoticeError>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, String>>,
}

impl<'a> Notice<'a> {
    /// Makes it easy to construct a new Notice
    ///
    /// ```
    /// use airbrake::{Context, Notice};
    ///
    /// let context = Context::builder();
    /// let notice = Notice::builder()
    ///     .context(context)
    ///     .build();
    /// ```
    pub fn builder() -> NoticeBuilder<'a> {
        NoticeBuilder::new()
    }

    pub fn send(self) -> Result<(), AirbrakeClientError> {
        match self.client {
            Some(c) => {
                debug!("Sending via notice client");
                c.notify(self)
            }
            None => Err(AirbrakeClientError::NoticeClientNotSet),
        }
    }
}

impl<'a> From<Notice<'a>> for Value {
    fn from(notice: Notice<'a>) -> Value {
        serde_json::json!(notice)
    }
}

#[cfg(test)]
mod tests {
    use super::{Context, Notice};
    use serde_json::{self, Value};
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn notice_default() {
        let notice = Notice::builder().build();
        let expected_json = r#"
        {
            "errors": []
        }
        "#;
        assert_eq!(Value::from_str(expected_json).unwrap(), Value::from(notice));
    }

    #[test]
    fn notice_with_add_environment() {
        let notice = Notice::builder().add_environment("foo", "bar").build();
        let expected_json = r#"
        {
            "errors": [],
            "environment": {
                "foo": "bar"
            }
        }
        "#;
        assert_eq!(Value::from_str(expected_json).unwrap(), Value::from(notice));
    }

    #[test]
    fn notice_with_set_environment() {
        let mut hashmap = HashMap::new();
        hashmap.insert("foo".to_string(), "bar".to_string());
        let notice = Notice::builder().environment(hashmap).build();
        let expected_json = r#"
        {
            "errors": [],
            "environment": {
                "foo": "bar"
            }
        }
        "#;
        assert_eq!(Value::from_str(expected_json).unwrap(), Value::from(notice));
    }

    #[test]
    fn notice_with_add_session() {
        let notice = Notice::builder().add_session("foo", "bar").build();
        let expected_json = r#"
        {
            "errors": [],
            "session": {
                "foo": "bar"
            }
        }
        "#;
        assert_eq!(Value::from_str(expected_json).unwrap(), Value::from(notice));
    }

    #[test]
    fn notice_with_set_session() {
        let mut hashmap = HashMap::new();
        hashmap.insert("foo".to_string(), "bar".to_string());
        let notice = Notice::builder().session(hashmap).build();
        let expected_json = r#"
        {
            "errors": [],
            "session": {
                "foo": "bar"
            }
        }
        "#;
        assert_eq!(Value::from_str(expected_json).unwrap(), Value::from(notice));
    }

    #[test]
    fn notice_with_add_param() {
        let notice = Notice::builder().add_param("foo", "bar").build();
        let expected_json = r#"
        {
            "errors": [],
            "params": {
                "foo": "bar"
            }
        }
        "#;
        assert_eq!(Value::from_str(expected_json).unwrap(), Value::from(notice));
    }

    #[test]
    fn notice_with_set_params() {
        let mut hashmap = HashMap::new();
        hashmap.insert("foo".to_string(), "bar".to_string());
        let notice = Notice::builder().params(hashmap).build();
        let expected_json = r#"
        {
            "errors": [],
            "params": {
                "foo": "bar"
            }
        }
        "#;
        assert_eq!(Value::from_str(expected_json).unwrap(), Value::from(notice));
    }

    #[test]
    fn notice_context_default() {
        let context = Context::builder();
        let notice = Notice::builder().context(context).build();
        let expected_json = r#"
        {
            "errors": [],
            "context": {
                "notifier": {
                    "name": "airbrake-rust",
                    "version": "0.2.0",
                    "url": "https://github.com/airbrake/airbrake-rust"
                }
            }
        }
        "#;
        assert_eq!(Value::from_str(expected_json).unwrap(), Value::from(notice));
    }
}
