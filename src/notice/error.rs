use super::NoticeFrame;
use crate::backtrace::Backtrace;
use std::error::Error;
use std::panic::PanicInfo;

#[derive(Default, Clone)]
pub struct NoticeErrorBuilder {
    pub name: String,
    pub message: Option<String>,
    pub backtrace: Option<Vec<NoticeFrame>>,
}

impl NoticeErrorBuilder {
    pub fn new(name: &str) -> NoticeErrorBuilder {
        NoticeErrorBuilder {
            name: name.to_string(),
            message: None,
            backtrace: None,
        }
    }

    pub fn message(&mut self, message: &str) -> &mut NoticeErrorBuilder {
        self.message = Some(message.to_string());
        self
    }

    // TODO: Maybe this should be renamed to `trace` and the `raw_backtrace` renamed to `backetrace`?
    pub fn backtrace(&mut self, backtrace: Vec<NoticeFrame>) -> &mut NoticeErrorBuilder {
        self.backtrace = Some(backtrace);
        self
    }

    pub fn raw_backtrace(&mut self, backtrace: &Backtrace) -> &mut NoticeErrorBuilder {
        self.backtrace = Some(NoticeFrame::from_backtrace(backtrace));
        self
    }

    pub fn build(&self) -> NoticeError {
        NoticeError::new(&self.name, self.message.clone(), self.backtrace.clone())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NoticeError {
    #[serde(rename = "type")]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrace_frames: Option<Vec<NoticeFrame>>,
}

impl NoticeError {
    pub fn new(
        name: &str,
        message: Option<String>,
        backtrace_frames: Option<Vec<NoticeFrame>>,
    ) -> NoticeError {
        NoticeError {
            name: name.to_string(),
            message,
            backtrace_frames,
        }
    }

    pub fn builder(name: &str) -> NoticeErrorBuilder {
        NoticeErrorBuilder::new(name)
    }

    pub fn from_panic_backtrace(panic_info: &PanicInfo, backtrace: &Backtrace) -> NoticeError {
        // TODO: PanicInfo has a `message()` on nightly which might be easier to work with
        // here, but for now we'll just deal with `payload()`
        let opt_message: Option<&str> = panic_info.payload().downcast_ref::<&str>().copied();
        let mut builder = NoticeError::builder("panic");
        builder.raw_backtrace(&backtrace);
        if let Some(message) = opt_message {
            builder.message(message);
        }
        builder.build()
    }
}

impl<'a, E: Error> From<E> for NoticeError {
    fn from(error: E) -> NoticeError {
        let name = format!("{:?}", error)
            .split_whitespace()
            .next()
            .unwrap()
            .to_string();
        let message = Some(format!("{}", error));
        let backtrace = None;

        NoticeError::new(&name, message, backtrace)
    }
}
