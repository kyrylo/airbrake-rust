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
//! extern crate airbrake;
//!
//! use std::num::ParseIntError;
//!
//! fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
//!    number_str.parse::<i32>().map(|n| 2 * n)
//! }
//!
//! fn main() {
//!     let mut airbrake = airbrake::configure(|config| {
//!         config.project_id("113743".to_owned());
//!         config.project_key("81bbff95d52f8856c770bb39e827f3f6".to_owned());
//!     });
//!
//!     match double_number("NOT A NUMBER") {
//!         Ok(n) => assert_eq!(n, 20),
//!         // Asynchronously sends the error to the dashboard.
//!         Err(err) => {
//!             let notice = airbrake::Notice::builder()
//!                 .add_error(err)
//!                 .build();
//!             airbrake.notify(notice)
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
//!     config.project_id("113743".to_owned());
//!     config.project_key("81bbff95d52f8856c770bb39e827f3f6".to_owned());
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
//!     config.project_id("113743".to_owned());
//!     config.project_key("81bbff95d52f8856c770bb39e827f3f6".to_owned());
//!     // Setting the host
//!     config.host("http://localhost:8080".to_owned());
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
//!     config.project_id("113743".to_owned());
//!     config.project_key("81bbff95d52f8856c770bb39e827f3f6".to_owned());
//!     // Setting the proxy
//!     config.proxy("127.0.0.1:8080".to_owned());
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
//!     config.project_id("123".to_owned());
//!     config.project_key("321".to_owned());
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
//!     config.project_id("123".to_owned());
//!     config.project_key("321".to_owned());
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


extern crate tokio;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate log;

mod config;
mod client;
mod notice;

pub use client::AirbrakeClient;
pub use config::{AirbrakeConfig, AirbrakeConfigBuilder};
pub use notice::*;

/// Configures an Airbrake notifier.
///
/// # Examples
///
/// ```
/// let mut airbrake = airbrake::configure(|config| {
///     config.project_id("113743".to_owned());
///     config.project_key("81bbff95d52f8856c770bb39e827f3f6".to_owned());
/// });
/// ```
pub fn configure<F>(builder_callback: F) -> AirbrakeClient
    where F: Fn(&mut AirbrakeConfigBuilder)
{
    let config = AirbrakeConfig::builder()
        .configure(builder_callback)
        .build()
        .unwrap();
    AirbrakeClient::new(config)
}
