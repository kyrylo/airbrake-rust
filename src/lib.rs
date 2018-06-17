extern crate backtrace;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate mockito;

mod notifier;
mod notice;

pub use self::notifier::{Config, Notifier};
pub use self::notice::{Notice, Param};
