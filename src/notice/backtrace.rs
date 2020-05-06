
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct NoticeBacktrace {
    pub file: String,
    pub line: i32,
    pub function: String,
    pub code: Option<HashMap<i32, String>>
}
