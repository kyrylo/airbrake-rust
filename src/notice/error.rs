
use std::error::Error;
use super::NoticeBacktraceFrame;

#[derive(Debug, Serialize)]
pub struct NoticeError {
    #[serde(rename="type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrace: Option<Vec<NoticeBacktraceFrame>>
}

impl NoticeError {
    pub fn new(name: String, message: Option<String>, backtrace: Option<Vec<NoticeBacktraceFrame>>) -> NoticeError {
        NoticeError {
            type_: name,
            message: message,
            backtrace: backtrace
        }
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

#[cfg(test)]
mod tests {
    use backtrace::{Backtrace, BacktraceFrame};
    use super::super::{NoticeBacktrace, NoticeBacktraceFrame};

    #[test]
    fn backtrace_contains_current_function_frame() {
        let function_name: String = "airbrake::notice::error::tests::backtrace_contains_current_function_frame".to_string();
        // This test builds a new backtrace object and asserts that
        // the current function exists somewhere in the resulting
        // list of frames.
        let backtrace = Backtrace::new();
        let notice_backtrace = NoticeBacktrace::from(backtrace);

        let has_function_name: bool = notice_backtrace.frames().iter()
            .fold(false, |acc: bool, frame: &NoticeBacktraceFrame| {
                acc || frame.function.contains(&function_name)
            });
        assert!(has_function_name);
    }
}
