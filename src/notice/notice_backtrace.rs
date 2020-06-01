use crate::backtrace::{Backtrace, BacktraceFrame, BacktraceSymbol};
use std::collections::HashMap;

#[derive(Debug, Serialize, Clone)]
pub struct NoticeFrame {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,

    /// TODO: Support for column number not yet implemented
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,

    /// TODO: Support for surrounding code not implemented
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<HashMap<i32, String>>,
}

impl NoticeFrame {
    /// A single BacktraceFrame can contain multiple "symbols" which are effectively
    /// different scopes. The frame is a context of a single line, but that
    /// single line may have multiple scopes. The Backtrace/Airbrake relationship
    /// is 1-to-1 between a Backtrace Symbols (not Backtrace Frame) and a Airbrake Frame.
    fn unroll_frame_symbols(frame: &BacktraceFrame) -> Vec<NoticeFrame> {
        frame.symbols().iter().map(NoticeFrame::from).collect()
    }

    pub fn from_backtrace(backtrace: &Backtrace) -> Vec<NoticeFrame> {
        let mut frames: Vec<NoticeFrame> = vec![];
        for f in backtrace.frames() {
            frames.append(&mut NoticeFrame::unroll_frame_symbols(&f));
        }
        frames
    }
}

impl From<&BacktraceSymbol> for NoticeFrame {
    fn from(symbol: &BacktraceSymbol) -> NoticeFrame {
        let filename = symbol
            .filename()
            .map(|sfn| sfn.to_str())
            .map(|sfn| sfn.unwrap().to_string());
        let function_name = symbol.name().map(|sn| sn.to_string());

        NoticeFrame {
            file: filename,
            line: symbol.lineno(),
            column: None,
            function: function_name,
            code: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoticeFrame;
    use crate::backtrace::Backtrace;

    #[test]
    fn backtrace_contains_current_function_frame() {
        let function_name: String = "backtrace_contains_current_function_frame".to_string();
        // This test builds a new backtrace object and asserts that
        // the current function exists somewhere in the resulting
        // list of frames.
        let backtrace = Backtrace::new();
        let selected_frames: Vec<NoticeFrame> = NoticeFrame::from_backtrace(&backtrace)
            .into_iter()
            .filter(|frame| match frame.function.as_ref() {
                Some(inner) => inner.contains(&function_name),
                None => false,
            })
            .collect();

        assert_gt!(selected_frames.len(), 0);
    }

    #[test]
    fn backtrace_unrolls_multiple_symboles() {
        let function_name: String = "backtrace_unrolls_multiple_symboles".to_string();
        let nested_frame_line: u32 = 89;
        // This backtrace is generated from within a nested enclosure so
        // that the backtraces creates a single frame with two symboles
        let fn_backtrace = || (|| Backtrace::new())();
        let backtrace = fn_backtrace();
        let selected_frames: Vec<NoticeFrame> = NoticeFrame::from_backtrace(&backtrace)
            .into_iter()
            .filter(|frame| match (frame.function.as_ref(), frame.line) {
                (Some(f), Some(l)) => f.contains(&function_name) && l == nested_frame_line,
                _ => false,
            })
            .collect();

        assert_gt!(selected_frames.len(), 1);
    }
}
