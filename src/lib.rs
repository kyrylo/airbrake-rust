//! Airbrake Rust is an [Airbrake][airbrake.io] notifier library for the Rust
//! Programming language. The library provides minimalist API that enables the
//! ability to send Rust errors to the Airbrake dashboard.
//!
//! Installation
//! ------------
//!
//! Add the crate to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! airbrake = "0.1"
//! ```
//!
//! Examples
//! --------
//!
//! ### Basic example
//!
//! This is the minimal example that you can use to test Airbrake Rust with your
//! project:
//!
//! ```
//! fn double_number(number_str: &str) -> Result<i32, std::num::ParseIntError> {
//!    number_str.parse::<i32>().map(|n| 2 * n)
//! }
//!
//! fn main() {
//!     let mut airbrake = airbrake::configure(|config| {
//!         config.project_id("113743");
//!         config.project_key("81bbff95d52f8856c770bb39e827f3f6");
//!     });
//!
//!     match double_number("NOT A NUMBER") {
//!         Ok(n) => assert_eq!(n, 20),
//!         // Asynchronously sends the error to the dashboard.
//!         Err(err) => {
//!             airbrake.new_notice_builder()
//!                 .add_error(err)
//!                 .build()
//!                 .send();
//!         }
//!     }
//! }
//! ```
//!
//! Configuration
//! -------------
//!
//! ### project_key & project_id
//!
//! You **must** set both `project_id` & `project_key`.
//!
//! To find your `project_id` and `project_key` navigate to your project's _General
//! Settings_ and copy the values from the right sidebar.
//!
//! ![][project-idkey]
//!
//! ```
//! let mut airbrake = airbrake::configure(|config| {
//!     config.project_id("113743");
//!     config.project_key("81bbff95d52f8856c770bb39e827f3f6");
//! });
//! ```
//!
//! ### host
//!
//! By default, it is set to `https://airbrake.io`. A `host` is a web address
//! containing a scheme ("http" or "https"), a host and a port. You can omit the
//! port (80 will be assumed).
//!
//! ```
//! let mut airbrake = airbrake::configure(|config| {
//!     // Project ID & Key are required
//!     config.project_id("113743");
//!     config.project_key("81bbff95d52f8856c770bb39e827f3f6");
//!     // Setting the host
//!     config.host("http://localhost:8080");
//! });
//! ```
//!
//! ### proxy
//!
//! If your server is not able to directly reach Airbrake, you can use proxy
//! support. By default, Airbrake Rust uses direct connection. Note: proxy
//! authentication is not supported yet.
//!
//! ```
//! let mut airbrake = airbrake::configure(|config| {
//!     // Project ID & Key are required
//!     config.project_id("113743");
//!     config.project_key("81bbff95d52f8856c770bb39e827f3f6");
//!     // Setting the proxy
//!     config.proxy("127.0.0.1:8080");
//! });
//! ```
//!
//! API
//! ---
//!
//! ## airbrake
//!
//! #### airbrake.notify
//!
//! Sends an error to Airbrake *asynchronously*. `error` must implement the
//! [`std::error::Error`][stderror] trait. Returns `()`.
//!
//! ```
//! let mut airbrake = airbrake::configure(|config| {
//!     config.project_id("123");
//!     config.project_key("321");
//! });
//!
//! let err = std::io::Error::last_os_error();
//! let notice = airbrake::Notice::builder()
//!     .add_error(err)
//!     .build();
//! airbrake.notify(notice);
//! ```
//!
//! As the second parameter, accepts a hash with additional data. That data will be
//! displayed in the _Params_ tab in your project's dashboard.
//!
//! #### airbrake.notify
//!
//! Sends an error to Airbrake *synchronously*. `error` must implement the
//! [`std::error::Error`][stderror] trait. Returns
//! [`rustc_serialize::json::Json`][json-object]. Accepts the same
//! parameters as [`Airbrake.notify`](#airbrakenotify).
//!
//! ```
//! let mut airbrake = airbrake::configure(|config| {
//!     config.project_id("123");
//!     config.project_key("321");
//! });
//!
//! let err = std::io::Error::last_os_error();
//! let notice = airbrake::Notice::builder()
//!     .add_error(err)
//!     .build();
//! airbrake.notify(notice);
//! ```
//!
//! [airbrake.io]: https://airbrake.io
//! [notice-v3]: https://airbrake.io/docs/#create-notice-v3
//! [env_logger]: https://crates.io/crates/env_logger
//! [project-idkey]: https://s3.amazonaws.com/airbrake-github-assets/airbrake-ruby/project-id-key.png
//! [stderror]: https://doc.rust-lang.org/std/error
//! [json-object]: https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/enum.Json.html
//!
//!
//!
//!
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
//! let notice_error = NoticeError::new("foo", None, None);
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
//! Airbrake supports multiple errors being logged in a single notification,
//! so using `.add_error` and `.add_notice` will append to the list of errors
//! that contained. If you have multiple errors ready, you can add them all
//! at once using `.add_errors` or `.add_notices`, which accept iterators.
//!
//! ```
//! use std::error::Error;
//! use airbrake::{Notice, NoticeError};
//!
//! let my_error1 = NoticeError::new("foo", None, None);
//! let my_error2 = NoticeError::new("bar", None, None);
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
//! let notice_error = NoticeError::new("foo", None, None);
//! let notice = context.new_notice_builder()
//!     .add_notice(notice_error)
//!     .build();
//! ```
//!

#![warn(unused_extern_crates)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate more_asserts;

#[cfg(test)]
#[macro_use]
extern crate matches;

mod client;
mod context;
mod notice;

pub use backtrace;
pub use client::{AirbrakeClient, AirbrakeClientBuilder, AirbrakeClientError};
pub use context::{Context, ContextBuilder, ContextUser, ContextProperties, CONTEXT_NOTIFIER};
pub use notice::*;

/// Configures an Airbrake notifier.
///
/// # Examples
///
/// ```
/// let mut airbrake = airbrake::configure(|config| {
///     config.project_id("113743");
///     config.project_key("81bbff95d52f8856c770bb39e827f3f6");
/// });
/// ```
pub fn configure<F>(builder_callback: F) -> AirbrakeClient
where
    F: Fn(&mut AirbrakeClientBuilder),
{
    AirbrakeClient::builder()
        .configure(builder_callback)
        .build()
        .expect("Airbrake configuration failed")
}
