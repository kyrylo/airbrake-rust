extern crate hyper;
extern crate rustc_serialize;
#[macro_use]
extern crate log;

mod config;
mod notifier;
mod notice;

use notifier::Notifier;
use config::Config;

pub fn configure<F>(configurator: F) -> Notifier where F: Fn(&mut Config) {
    let mut config = Config::new();
    configurator(&mut config);
    Notifier::new(config)
}
