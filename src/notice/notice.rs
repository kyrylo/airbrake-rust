
use super::{
    Context,
    NoticeError
};

use serde_json;

use std::error::Error;
use std::collections::HashMap;
use std::string::ToString;
use hyper::body::Body;
use crate::AirbrakeConfig;

pub struct NoticeBuilder {
    pub errors: Vec<NoticeError>,
    pub context: Option<Context>,
    pub environment: Option<HashMap<String, String>>,
    pub session: Option<HashMap<String, String>>,
    pub params: Option<HashMap<String, String>>
}

impl NoticeBuilder {
    /// Set the environment on the NoticeBuilder
    pub fn new() -> NoticeBuilder {
        NoticeBuilder {
            errors: vec![],
            context: None,
            environment: None,
            session: None,
            params: None
        }
    }

    /// Add multiple NoticeErrors from an iterator
    pub fn add_notices<T: Iterator<Item = NoticeError>>(mut self, notice_errors: T) -> NoticeBuilder {
        self.errors.extend(notice_errors);
        self
    }

    /// Add a single NoticeError
    pub fn add_notice(mut self, notice_error: NoticeError) -> NoticeBuilder {
        self.errors.push(notice_error);
        self
    }

    /// Add multiple Errors from an iterator
    pub fn add_errors<T: Iterator<Item = E>, E: Error>(self, errors: T) -> NoticeBuilder {
        let notice_errors = errors
            .into_iter()
            .map(|x| x.into());
        self.add_notices(notice_errors)
    }

    /// Add a single Error
    pub fn add_error<E: Error>(self, error: E) -> NoticeBuilder {
        let notice_error = NoticeError::from(error);
        self.add_notice(notice_error.into())
    }

    /// Set the context on the NoticeBuilder
    pub fn context(mut self, context: Context) -> NoticeBuilder {
        self.context = Some(context);
        self
    }

    /// Set the environment on the NoticeBuilder
    pub fn environment(mut self, environment: HashMap<String, String>) -> NoticeBuilder {
        self.environment = Some(environment);
        self
    }

    /// Set the environment on the NoticeBuilder
    pub fn session(mut self, session: HashMap<String, String>) -> NoticeBuilder {
        self.session = Some(session);
        self
    }

    /// Set the environment on the NoticeBuilder
    pub fn params(mut self, params: HashMap<String, String>) -> NoticeBuilder {
        self.params = Some(params);
        self
    }

    /// Executes the command as a child process, which is returned.
    pub fn build(self) -> Notice {
        Notice {
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
impl<'a> From<Context> for NoticeBuilder {
    fn from(context: Context) -> NoticeBuilder {
        NoticeBuilder::new().context(context.clone())
    }
}

impl<'a, E: Error> From<E> for NoticeBuilder {
    fn from(error: E) -> NoticeBuilder {
        NoticeBuilder::new().add_error(error)
    }
}

#[derive(Debug, Serialize)]
pub struct Notice {
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

impl Notice {
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
    pub fn builder() -> NoticeBuilder {
        NoticeBuilder::new()
    }

    pub fn new<E: Error>(config: &AirbrakeConfig, error: E) -> Notice {
        let notice_error = NoticeError::from(error);
        NoticeBuilder::new()
            .add_notice(notice_error)
            .build()
        // Notice {
        //     context: Some(Context {
        //         notifier: &CONTEXT_NOTIFIER,
        //         operating_system: None,
        //         hostname: None,
        //         language: None,
        //         environment: None,
        //         severity: None,
        //         version: Some(config.app_version.clone()),
        //         url: None,
        //         root_directory: None,
        //         user: None,
        //         route: None,
        //         http_method: None
        //     }),
        //     environment: None,
        //     session: None,
        //     params: None
        // }
    }
}

impl Into<Body> for Notice {
    fn into(self) -> Body {
        Body::from(serde_json::json!(self).to_string())
    }
}
