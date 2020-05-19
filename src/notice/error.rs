
use std::error::Error;
use super::NoticeTrace;
use crate::backtrace::Backtrace;

pub struct NoticeErrorBuilder {
    pub name: String,
    pub message: Option<String>,
    pub backtrace: Option<NoticeTrace>
}

impl NoticeErrorBuilder {
    pub fn new(name: &str) -> NoticeErrorBuilder {
        NoticeErrorBuilder {
            name: name.to_string(),
            message: None,
            backtrace: None
        }
    }

    pub fn message(mut self, message: String) -> NoticeErrorBuilder {
        self.message = Some(message);
        self
    }

    pub fn backtrace(mut self, backtrace: NoticeTrace) -> NoticeErrorBuilder {
        self.backtrace = Some(backtrace);
        self
    }

    pub fn raw_backtrace(mut self, backtrace: Backtrace) -> NoticeErrorBuilder {
        self.backtrace = Some(NoticeTrace::from(backtrace));
        self
    }

    pub fn build(self) -> NoticeError {
        NoticeError::new(self.name, self.message, self.backtrace)
    }
}

#[derive(Debug, Serialize)]
pub struct NoticeError {
    #[serde(rename="type")]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrace: Option<NoticeTrace>
}

impl NoticeError {
    pub fn new(name: String, message: Option<String>, backtrace: Option<NoticeTrace>) -> NoticeError {
        NoticeError {
            name: name,
            message: message,
            backtrace: backtrace
        }
    }

    pub fn builder(name: &str) -> NoticeErrorBuilder {
        NoticeErrorBuilder::new(name)
    }
}

impl<'a, E: Error> From<E> for NoticeError {
    fn from(error: E) -> NoticeError {
        let name = format!("{:?}", error).split_whitespace().next().unwrap().to_owned();
        let message = Some(format!("{}", error));
        let backtrace = None;

        NoticeError::new(name, message, backtrace)
    }
}
