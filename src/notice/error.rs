
use std::error::Error;
use std::panic::PanicInfo;
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

    pub fn message(mut self, message: &str) -> NoticeErrorBuilder {
        self.message = Some(message.to_string());
        self
    }

    // TODO: Maybe this should be renamed to `trace` and the `raw_backtrace` renamed to `backetrace`?
    pub fn backtrace(mut self, backtrace: NoticeTrace) -> NoticeErrorBuilder {
        self.backtrace = Some(backtrace);
        self
    }

    pub fn raw_backtrace(mut self, backtrace: &Backtrace) -> NoticeErrorBuilder {
        self.backtrace = Some(NoticeTrace::from(backtrace));
        self
    }

    pub fn build(self) -> NoticeError {
        NoticeError::new(&self.name, self.message, self.backtrace)
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
    pub fn new(name: &str, message: Option<String>, backtrace: Option<NoticeTrace>) -> NoticeError {
        NoticeError {
            name: name.to_string(),
            message,
            backtrace
        }
    }

    pub fn builder(name: &str) -> NoticeErrorBuilder {
        NoticeErrorBuilder::new(name)
    }

    pub fn from_panic_backtrace(panic_info: &PanicInfo, backtrace: &Backtrace) -> NoticeError {
        // TODO: PanicInfo has a `message()` on nightly which might be easier to work with
        // here, but for now we'll just deal with `payload()`
        let message: String = panic_info.payload()
            .downcast_ref::<String>()
            .map(|s| s.to_string())
            .or_else(|| Some( "None".to_string() ))
            .unwrap();
        NoticeError::builder("panic")
            .message(&message)
            .raw_backtrace(backtrace)
            .build()
    }
}

impl<'a, E: Error> From<E> for NoticeError {
    fn from(error: E) -> NoticeError {
        let name = format!("{:?}", error).split_whitespace().next().unwrap().to_string();
        let message = Some(format!("{}", error));
        let backtrace = None;

        NoticeError::new(&name, message, backtrace)
    }
}
