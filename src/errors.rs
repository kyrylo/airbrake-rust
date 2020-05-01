
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct NoticeError {
    #[serde(rename="type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrace: Option<Vec<NoticeBacktrace>>
}

#[derive(Debug, Serialize)]
pub struct NoticeBacktrace {
    pub file: String,
    pub line: i32,
    pub function: String,
    pub code: Option<HashMap<i32, String>>
}

