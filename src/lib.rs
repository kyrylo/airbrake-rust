//! Airbrake Rust is notifier library for the Rust Programming language. The
//! library provides minimalist API that enables the ability to send Rust errors
//! to the Airbrake dashboard.
//!
//! # Installation
//!
//! Add the crate to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! airbrake = "0.1"
//! ```
//!
//! # Examples
//!
//! ### Basic example
//!
//! This is the minimal example that you can use to test Airbrake Rust with your
//! project:
//!
//! ```rust
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
//!         config.project_id = "113743".to_owned();
//!         config.project_key = "81bbff95d52f8856c770bb39e827f3f6".to_owned();
//!     });
//!
//!     match double_number("NOT A NUMBER") {
//!         Ok(n) => assert_eq!(n, 20),
//!         Err(err) => airbrake.notify(err)
//!     }
//!
//!     airbrake.close();
//! }
//! ```

extern crate hyper;
extern crate rustc_serialize;
#[macro_use]
extern crate log;

mod config;
mod notifier;
mod notice;
mod async_sender;
mod sync_sender;

use notifier::Notifier;
use config::Config;

/// Configures an Airbrake notifier.
///
/// # Examples
///
/// ```
/// let mut airbrake = airbrake::configure(|config| {
///     config.project_id = "113743".to_owned();
///     config.project_key = "81bbff95d52f8856c770bb39e827f3f6".to_owned();
/// });
pub fn configure<F>(configurator: F) -> Notifier
    where F: Fn(&mut Config)
{
    let mut config = Config::new();
    configurator(&mut config);
    Notifier::new(config)
}
