use backtrace::Backtrace;

use std::error::Error;
use std::collections::HashMap;
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
    // file: String,
    // line: String,
    function: String,
}

impl Error for Notice {
    fn description(&self) -> &str {
        "Notice: hi"
    }
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
        let mut stack_frames = Vec::new();

        for frame in backtrace.frames() {
            let symbols = frame.symbols();
            if symbols.is_empty() {
                continue;
            }

            if let Some(function_name) = symbols[0].name() {
                stack_frames.push(StackFrame {
                    function: String::from(function_name.as_str().unwrap()),
                });
            }
        }
        self.errors[0].backtrace = Some(stack_frames);
        self
    }

    pub fn set_params(mut self, params: HashMap<String, Param>) -> Self {
        self.params = params;
        self
    }
}
