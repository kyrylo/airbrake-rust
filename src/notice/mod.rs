
//! The Notice module contains the various structs that make up an Airbrake
//! Notice. A Notice is primarily contains a vector of NoticeErrors, which
//! is the structure that represents the error itself. Other parts of the of
//! Notice are Context, Environment, Session and Parameters.
//!
//! At simplest, a Notice can be constructed using the notice builder,
//! allowing you to set context and errors as needed.
//!
//! ```
//! use airbrake::{Notice, NoticeError};
//!
//! let notice_error = NoticeError::new("foo".to_owned(), None, None);
//! let notice = Notice::builder()
//!     .add_notice(notice_error)
//!     .build();
//! ```
//!
//! NoticeError implements From<Error>, so you can use `.into()` to construct
//! instances directly from anything that implements Error.
//!
//! ```
//! use std::error::Error;
//! use std::fmt::{Display, Formatter, Result};
//! use airbrake::{Notice, NoticeError};
//!
//! #[derive(Debug)]
//! struct MyError;
//! impl Error for MyError {}
//! impl Display for MyError {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "") }
//! }
//! let my_error = MyError {};
//!
//! let ne: NoticeError = my_error.into();
//! ```
//!
//! Typically you won't need to work with the NoticeError directly, since you
//! can add errors to a Notice using the `.add_error` function.
//!
//! ```
//! use std::error::Error;
//! use std::fmt::{Display, Formatter, Result};
//! use airbrake::{Notice, NoticeError};
//!
//! #[derive(Debug)]
//! struct MyError;
//! impl Error for MyError {}
//! impl Display for MyError {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "") }
//! }
//! let my_error = MyError {};
//!
//! let notice = Notice::builder()
//!     .add_error(my_error)
//!     .build();
//! ```
//!
//! Backtraces are also supported in error reporting. You will typically have
//! a raw backtrace created by the backtrace crate. These sort of backtraces
//! can be added using the `raw_backtrace` builder method.
//! ```
//! use airbrake::{Notice, NoticeError};
//! use airbrake::backtrace::Backtrace;
//!
//! let notice_error = NoticeError::builder("foo")
//!     .raw_backtrace(&Backtrace::new())
//!     .build();
//! let notice = Notice::builder()
//!     .add_notice(notice_error)
//!     .build();
//! ```
//!
//! You can also include a backtrace while building a Notice based on an
//! error, however since backtraces aren't part of the Error trait, you'll
//! have to provide it using the add_error_with_backtrace() function:
//!
//! ```
//! use std::error::Error;
//! use std::fmt::{Display, Formatter, Result};
//! use backtrace;
//! use airbrake::{Notice, NoticeError};
//! use airbrake::backtrace::Backtrace;
//!
//! #[derive(Debug)]
//! struct MyError;
//! impl Error for MyError {}
//! impl Display for MyError {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "") }
//! }
//! let my_error = MyError {};
//! let my_backtrace = Backtrace::new();
//!
//! let notice = Notice::builder()
//!     .add_error_with_backtrace(my_error, my_backtrace)
//!     .build();
//! ```
//!
//! Airbreak supports multiple errors being logged in a single notification,
//! so using `.add_error` and `.add_notice` will append to the list of errors
//! that contained. If you have multiple errors ready, you can add them all
//! at once using `.add_errors` or `.add_notices`, which accept iterators.
//!
//! ```
//! use std::error::Error;
//! use airbrake::{Notice, NoticeError};
//!
//! let my_error1 = NoticeError::new("foo".to_owned(), None, None);
//! let my_error2 = NoticeError::new("bar".to_owned(), None, None);
//! let error_list = vec![my_error1, my_error2].into_iter();
//! let notice = Notice::builder()
//!     .add_notices(error_list)
//!     .build();
//! ```
//!
//! The Context struct represents the context the service is running in, like
//! operating system details, application version and other similar data.
//! Information within the Context is typically static, and doesn't change over
//! the runtime of the service. If you are using a Context, it makes more sense
//! to build Notices from the context rather than manually adding the context to
//! each Notice you create.
//!
//! ```
//! use airbrake::{NoticeError, Context};
//!
//! let context = Context::builder().build();
//!
//! let notice_error = NoticeError::new("foo".to_owned(), None, None);
//! let notice = context.new_notice_builder()
//!     .add_notice(notice_error)
//!     .build();
//! ```
//!

mod context;
mod error;
mod notice_backtrace;
mod notice;

pub use notice::{
    Notice,
    NoticeBuilder
};
pub use context::{
    Context,
    ContextBuilder,
    ContextUser,
    CONTEXT_NOTIFIER
};
pub use error::{
    NoticeError,
    NoticeErrorBuilder
};
pub use notice_backtrace::{NoticeTrace, NoticeFrame};
