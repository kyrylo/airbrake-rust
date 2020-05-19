
use std::error::Error;
use super::NoticeTrace;

#[derive(Debug, Serialize)]
pub struct NoticeError {
    #[serde(rename="type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrace: Option<NoticeTrace>
}

impl NoticeError {
    pub fn new(name: String, message: Option<String>, backtrace: Option<NoticeTrace>) -> NoticeError {
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
