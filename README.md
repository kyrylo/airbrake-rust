Airbrake Rust
=============

[![Build Status](https://travis-ci.org/kyrylo/airbrake-rust.svg?branch=master)](https://travis-ci.org/kyrylo/airbrake-rust)

**/!\ The project is in the alpha stage /!\**

* [Airbrake Rust README](https://github.com/kyrylo/airbrake-rust)

Introduction
------------

_Airbrake Rust_ is an [Airbrake][airbrake.io] notifier library for the Rust
Programming language. The library provides minimalist API that enables the
ability to send Rust errors to the Airbrake dashboard.

Key features
------------

* Uses the new Airbrake JSON API (v3)<sup>[[link][notice-v3]]</sup>
* Simple, consistent and easy-to-use library API<sup>[[link](#api)]</sup>
* Awesome performance (check out our benchmarks)<sup>[[link](#running-benchmarks)
* Asynchronous error reporting<sup>[[link](#asynchronous-airbrake-options)]</sup>
* Logging support via env_logger<sup>[[link][env_logger]]</sup>
* Support for proxying<sup>[[link](#proxy)]</sup>
* Support for environments<sup>[[link](#environment)]</sup>
* Filters support (filter out sensitive or unwanted data that shouldn't be sent)<sup>[[link](#airbrakeadd_filter)]</sup>
* Ability to ignore errors based on any condition<sup>[[link](#airbrakeadd_filter)]</sup>
* SSL support (all communication with Airbrake is encrypted by default)

Installation
------------

### Cargo

Add the crate to your Cargo.toml:

```toml
[dependencies]
airbrake = "0.1"
```

Examples
--------

### Basic example

This is the minimal example that you can use to test Airbrake Rust with your
project:


```rust
extern crate airbrake;

use std::num::ParseIntError;

fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
   number_str.parse::<i32>().map(|n| 2 * n)
}

fn main() {
    let mut airbrake = airbrake::configure(|config| {
        config.project_id = "113743".to_owned();
        config.project_key = "81bbff95d52f8856c770bb39e827f3f6".to_owned();
    });

    match double_number("NOT A NUMBER") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => airbrake.notify(err),
    }

    airbrake.close();
}
```

[airbrake.io]: https://airbrake.io
[notice-v3]: https://airbrake.io/docs/#create-notice-v3
[env_logger]: https://crates.io/crates/env_logger
