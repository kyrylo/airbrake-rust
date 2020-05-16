
use super::{
    Context,
    NoticeError
};

use serde_json::{self, Value};

use std::error::Error;
use std::collections::HashMap;
use std::string::ToString;
use hyper::body::Body;
use crate::AirbrakeConfig;
use crate::AirbrakeClient;

pub struct NoticeBuilder<'a> {
    pub client: Option<&'a AirbrakeClient>,
    pub errors: Vec<NoticeError>,
    pub context: Option<Context>,
    pub environment: Option<HashMap<String, String>>,
    pub session: Option<HashMap<String, String>>,
    pub params: Option<HashMap<String, String>>
}

impl<'a> NoticeBuilder<'a> {
    /// Set the environment on the NoticeBuilder
    pub fn new() -> NoticeBuilder<'a> {
        NoticeBuilder {
            client: None,
            errors: vec![],
            context: None,
            environment: None,
            session: None,
            params: None
        }
    }

    pub fn set_client(mut self, client: &'a AirbrakeClient) -> NoticeBuilder<'a> {
        self.client = Some(client);
        self
    }

    /// Add multiple NoticeErrors from an iterator
    pub fn add_notices<T: Iterator<Item = NoticeError>>(mut self, notice_errors: T) -> NoticeBuilder<'a> {
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
        let notice_errors = errors
            .into_iter()
            .map(|x| x.into());
        self.add_notices(notice_errors)
    }

    /// Add a single Error
    pub fn add_error<E: Error>(self, error: E) -> NoticeBuilder<'a> {
        let notice_error = NoticeError::from(error);
        self.add_notice(notice_error.into())
    }

    /// Set the context on the NoticeBuilder
    pub fn context(mut self, context: Context) -> NoticeBuilder<'a> {
        self.context = Some(context);
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
    ///     .add_environment("PORT".to_owned(), "443".to_owned())
    ///     .add_environment("CODE_NAME".to_owned(), "gorilla".to_owned())
    ///     .build();
    /// ```
    pub fn add_environment(mut self, key: String, value: String) -> NoticeBuilder<'a> {
        self.environment = self.environment
            .or_else(|| Some(HashMap::new()))
            .and_then(|mut h| {
                h.insert(key, value);
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
    ///     .add_session("basketId".to_owned(), "123".to_owned())
    ///     .add_session("userId".to_owned(), "456".to_owned())
    ///     .build();
    /// ```
    pub fn add_session(mut self, key: String, value: String) -> NoticeBuilder<'a> {
        self.session = self.session
            .or_else(|| Some(HashMap::new()))
            .and_then(|mut h| {
                h.insert(key, value);
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
    ///     .add_param("page".to_owned(), "3".to_owned())
    ///     .add_param("sort".to_owned(), "name".to_owned())
    ///     .add_param("direction".to_owned(), "asc".to_owned())
    ///     .build();
    /// ```
    pub fn add_param(mut self, key: String, value: String) -> NoticeBuilder<'a> {
        self.params = self.params
            .or_else(|| Some(HashMap::new()))
            .and_then(|mut h| {
                h.insert(key, value);
                Some(h)
            });
        self
    }

    /// Executes the command as a child process, which is returned.
    pub fn build(self) -> Notice<'a> {
        Notice {
            client: self.client,
            errors: self.errors,
            context: self.context,
            environment: self.environment,
            session: self.session,
            params: self.params
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
/// let context = Context::builder().build();
/// let notice_builder = NoticeBuilder::from(context);
/// ```
impl<'a, 'b> From<Context> for NoticeBuilder<'b> {
    fn from(context: Context) -> NoticeBuilder<'b> {
        NoticeBuilder::new().context(context.clone())
    }
}

impl<'a, 'b, E: Error> From<E> for NoticeBuilder<'b> {
    fn from(error: E) -> NoticeBuilder<'b> {
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
    pub params: Option<HashMap<String, String>>
}

impl<'a> Notice<'a> {
    /// Makes it easy to construct a new Notice
    ///
    /// ```
    /// use airbrake::{Context, Notice};
    ///
    /// let context = Context::builder().build();
    /// let notice = Notice::builder()
    ///     .context(context)
    ///     .build();
    /// ```
    pub fn builder() -> NoticeBuilder<'a> {
        NoticeBuilder::new()
    }

    pub fn new<E: Error>(config: &AirbrakeConfig, error: E) -> Notice<'a> {
        let notice_error = NoticeError::from(error);
        NoticeBuilder::new()
            .add_notice(notice_error)
            .build()
    }

    pub fn send(self) {
        self.client.and_then(|c| {
            c.notify(self);
            Some(c)
        });
    }
}

impl<'a> From<Notice<'a>> for Value {
    fn from(notice: Notice<'a>) -> Value {
        serde_json::json!(notice)
    }
}

impl<'a> Into<Body> for Notice<'a> {
    fn into(self) -> Body {
        Body::from(Value::from(self).to_string())
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::collections::HashMap;
    use serde_json::{self, Value};
    use hyper::body::Body;
    use super::{Notice, Context};

    #[test]
    fn notice_default() {
        let notice = Notice::builder().build();
        let expected_json = r#"
        {
            "errors": []
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            Value::from(notice)
        );
    }

    #[test]
    fn notice_with_add_environment() {
        let notice = Notice::builder()
            .add_environment("foo".to_owned(), "bar".to_owned())
            .build();
        let expected_json = r#"
        {
            "errors": [],
            "environment": {
                "foo": "bar"
            }
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            Value::from(notice)
        );
    }

    #[test]
    fn notice_with_set_environment() {
        let mut hashmap = HashMap::new();
        hashmap.insert("foo".to_owned(), "bar".to_owned());
        let notice = Notice::builder()
            .environment(hashmap)
            .build();
        let expected_json = r#"
        {
            "errors": [],
            "environment": {
                "foo": "bar"
            }
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            Value::from(notice)
        );
    }

    #[test]
    fn notice_with_add_session() {
        let notice = Notice::builder()
            .add_session("foo".to_owned(), "bar".to_owned())
            .build();
        let expected_json = r#"
        {
            "errors": [],
            "session": {
                "foo": "bar"
            }
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            Value::from(notice)
        );
    }

    #[test]
    fn notice_with_set_session() {
        let mut hashmap = HashMap::new();
        hashmap.insert("foo".to_owned(), "bar".to_owned());
        let notice = Notice::builder()
            .session(hashmap)
            .build();
        let expected_json = r#"
        {
            "errors": [],
            "session": {
                "foo": "bar"
            }
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            Value::from(notice)
        );
    }

    #[test]
    fn notice_with_add_param() {
        let notice = Notice::builder()
            .add_param("foo".to_owned(), "bar".to_owned())
            .build();
        let expected_json = r#"
        {
            "errors": [],
            "params": {
                "foo": "bar"
            }
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            Value::from(notice)
        );
    }

    #[test]
    fn notice_with_set_params() {
        let mut hashmap = HashMap::new();
        hashmap.insert("foo".to_owned(), "bar".to_owned());
        let notice = Notice::builder()
            .params(hashmap)
            .build();
        let expected_json = r#"
        {
            "errors": [],
            "params": {
                "foo": "bar"
            }
        }
        "#;
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            Value::from(notice)
        );
    }

    #[test]
    fn notice_context_default() {
        let context = Context::builder().build();
        let notice = Notice::builder()
            .context(context)
            .build();
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
        assert_eq!(
            Value::from_str(expected_json).unwrap(),
            Value::from(notice)
        );
    }
}