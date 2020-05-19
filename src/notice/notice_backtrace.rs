
use std::collections::HashMap;
use backtrace::BacktraceSymbol;

#[derive(Debug, Serialize)]
pub struct NoticeBacktraceFrame {
    pub file: String,
    pub function: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<HashMap<i32, String>>
}

impl From<&BacktraceSymbol> for NoticeBacktraceFrame {
    fn from(symbol: &BacktraceSymbol) -> NoticeBacktraceFrame {
        let filename = match symbol.filename() {
            Some( f ) => {
                f.to_str()
                .expect("Backtrace frame's filename was not valid unicode")
                .to_string()
            },
            None => "(anonymous)".to_string()
        };
        let function_name = symbol.name()
            .expect("Backtrace frame doesn't have a function name")
            .to_string();

        NoticeBacktraceFrame {
            file: filename,
            line: symbol.lineno(),
            function: function_name,
            code: None
        }
    }
}