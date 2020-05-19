
use std::collections::HashMap;
use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde_json::{self, Value};
use backtrace::{Backtrace, BacktraceFrame, BacktraceSymbol};

#[derive(Debug)]
pub struct NoticeBacktrace {
    frames: Vec<NoticeBacktraceFrame>
}

impl NoticeBacktrace {
    fn new(frames: Vec<NoticeBacktraceFrame>) -> NoticeBacktrace {
        NoticeBacktrace {
            frames: frames
        }
    }

    pub fn frames(&self) -> Vec<NoticeBacktraceFrame> {
        self.frames.clone()
    }
}

impl From<Backtrace> for NoticeBacktrace {
    fn from(backtrace: Backtrace) -> NoticeBacktrace {
        let mut frames: Vec<NoticeBacktraceFrame> = vec![];
        for f in backtrace.frames() {
            frames.append(
                &mut NoticeBacktraceFrame::unroll_frame_symbols(&f)
            );
        }
        NoticeBacktrace {
            frames: frames
        }
    }
}

impl Serialize for NoticeBacktrace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.frames.len()))?;
        for element in self.frames() {
            seq.serialize_element(&element)?;
        }
        seq.end()
    }
}

impl From<NoticeBacktrace> for Value {
    fn from(notice_backtrace: NoticeBacktrace) -> Value {
        serde_json::json!(notice_backtrace)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct NoticeBacktraceFrame {
    pub file: String,
    pub function: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<HashMap<i32, String>>
}

impl NoticeBacktraceFrame {
    fn unroll_frame_symbols(frame: &BacktraceFrame) -> Vec<NoticeBacktraceFrame> {
        frame.symbols().iter().map(NoticeBacktraceFrame::from).collect()
    }
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

#[cfg(test)]
mod tests {
    use std::convert::From;
    use backtrace::{Backtrace, BacktraceFrame};
    use super::{NoticeBacktrace, NoticeBacktraceFrame};

    #[test]
    fn backtrace_contains_current_function_frame() {
        let function_name: String = "backtrace_contains_current_function_frame".to_string();
        // This test builds a new backtrace object and asserts that
        // the current function exists somewhere in the resulting
        // list of frames.
        let backtrace = Backtrace::new();
        let selected_frames: Vec<NoticeBacktraceFrame> = NoticeBacktrace::from(backtrace)
            .frames()
            .into_iter()
            .filter(|f| {
                f.function.contains(&function_name)
            })
            .collect();

        assert_gt!(selected_frames.len(), 0);
    }

    #[test]
    fn backtrace_unrolls_multiple_symboles() {
        let function_name: String = "backtrace_unrolls_multiple_symboles".to_string();
        let nested_frame_line: u32 = 128;
        // This backtrace is generated from within a nested enclosure so
        // that the backtraces creates a single frame with two symboles
        let fn_backtrace = || { (|| { Backtrace::new() })() };
        let backtrace = fn_backtrace();
        let selected_frames: Vec<NoticeBacktraceFrame> = NoticeBacktrace::from(backtrace)
            .frames()
            .into_iter()
            .filter(|f| {
                f.function.contains(&function_name) &&
                f.line == Some(nested_frame_line)
            })
            .collect();

        assert_gt!(selected_frames.len(), 1);
    }

    #[test]
    fn json_backtrace_is_array_of_objects() {
        use std::str::FromStr;
        use std::collections::HashMap;
        use serde_json::{self, Value};

        let backtrace = Backtrace::new();
        let notice_backtrace = NoticeBacktrace::from(backtrace);
        let json_backtrace = Value::from(notice_backtrace);

        assert_matches!(json_backtrace, Value::Array(_));
        assert_matches!(json_backtrace[0], Value::Object(_));
    }
}