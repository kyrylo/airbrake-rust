
use std::collections::HashMap;
use backtrace::{Backtrace, BacktraceFrame, BacktraceSymbol};

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