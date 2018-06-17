use backtrace::Backtrace;

use std::error::Error;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct Notice {
    errors: [AirbrakeError; 1],
    params: HashMap<String, Param>,
}

#[derive(Serialize, Debug)]
struct AirbrakeError {
    #[serde(rename = "type")]
    type_: String,
    message: String,
    backtrace: Option<Vec<StackFrame>>,
}

#[derive(Serialize, Debug)]
pub enum Param {
    Int32(i32),
    String(String),
}

impl fmt::Display for Notice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Notice error")
    }
}

#[derive(Serialize, Debug)]
struct StackFrame {
    line: Option<u32>,
    file: Option<PathBuf>,
    function: Option<String>,
}

impl Notice {
    pub fn new<T: Error>(error: T) -> Self {
        Self {
            errors: [AirbrakeError {
                type_: String::from(format!("{:?}", error).split_whitespace().next().unwrap()),
                message: String::from(error.description()),
                backtrace: None,
            }; 1],
            params: HashMap::new(),
        }
    }

    pub fn set_backtrace(mut self, backtrace: Backtrace) -> Self {
        let mut frames = Vec::new();

        for frame in backtrace.frames() {
            // TODO: Add support for multiple symbols.
            // https://docs.rs/backtrace/0.3.8/backtrace/struct.BacktraceFrame.html#method.symbols
            if let Some(symbol) = frame.symbols().first() {
                let function = symbol
                    .name()
                    .and_then(|f| f.as_str())
                    .and_then(|f| Some(String::from(f)));

                frames.push(StackFrame {
                    line: symbol.lineno(),
                    file: symbol.filename().and_then(|f| Some(f.to_path_buf())),
                    function: function,
                });
            }
        }
        self.errors[0].backtrace = Some(frames);
        self
    }

    pub fn set_params(mut self, params: HashMap<String, Param>) -> Self {
        self.params = params;
        self
    }
}
