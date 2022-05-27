Airbrake Rust
=============

[![Build Status](https://travis-ci.org/kyrylo/airbrake-rust.svg?branch=master)](https://travis-ci.org/kyrylo/airbrake-rust)

**The project is in the alpha stage and I don't recommend using it in
production until it hits v1.0.0**. See the [v0.2.0 tag](https://github.com/kyrylo/airbrake-rust/tree/422147119de3ba16ca8915fef2ac8f50b74526bf) for documentation for the current latest release of this project.

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
* Awesome performance (check out our benchmarks)<sup>[[link](#running-benchmarks)]</sup>
* Asynchronous error reporting<sup>[[link](#asynchronous-airbrake-options)]</sup>
* Logging support via env_logger<sup>[[link][env_logger]]</sup>
* Support for proxying (WIP)<sup>[[link](#proxy)]</sup>
* Support for environments<sup>[[link](#environment)]</sup>
* Filters support (filter out sensitive or unwanted data that shouldn't be sent) (WIP)<sup>[[link](#airbrakeadd_filter)]</sup>
* Ability to ignore errors based on any condition (WIP)<sup>[[link](#airbrakeadd_filter)]</sup>
* SSL support (all communication with Airbrake is encrypted by default)

Installation
------------

### Cargo

Add the crate to your Cargo.toml:

```toml
[dependencies]
airbrake = "0.2"
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
        config.project_id("113743");
        config.project_key("81bbff95d52f8856c770bb39e827f3f6");
    });

    match double_number("NOT A NUMBER") {
        Ok(n) => assert_eq!(n, 20),
        // Asynchronously sends the error to the dashboard.
        Err(err) => {
            airbrake.new_notice_builder()
                .add_error(err)
                .build()
                .send();
        }
    }
}
```

Configuration
-------------

### project_key & project_id

You **must** set both `project_id` & `project_key`.

To find your `project_id` and `project_key` navigate to your project's _General
Settings_ and copy the values from the right sidebar.

![][project-idkey]

```rust
let mut airbrake = airbrake::configure(|config| {
    config.project_id("113743");
    config.project_key("81bbff95d52f8856c770bb39e827f3f6");
});
```

### host

By default, it is set to `https://app.airbrake.io`. A `host` is a web address
containing a scheme ("http" or "https"), a host and a port. You can omit the
port (80 will be assumed).

```rust
let mut airbrake = airbrake::configure(|config| {
    config.host("http://localhost:8080");
});
```

### proxy

If your server is not able to directly reach Airbrake, you can use proxy
support. By default, Airbrake Rust uses direct connection. Note: proxy
authentication is not supported yet.

```rust
let mut airbrake = airbrake::configure(|config| {
    config.proxy("127.0.0.1:8080");
});
```

### app_version

The version of your application that you can pass to differentiate errors
between multiple versions. It's not set by default.

```rust
let mut airbrake = airbrake::configure(|config| {
    config.version("1.0.0")
});
```

API
---

### airbrake

#### airbrake.notify

Sends an error to Airbrake *asynchronously*. `error` must implement the
[`std::error::Error`][stderror] trait. Returns `()`.

```rust
let mut airbrake = airbrake::configure(|config| {
    config.project_id = "123".to_owned();
    config.project_key = "321".to_owned();
});

airbrake.notify(std::io::Error::last_os_error());
```

[airbrake.io]: https://airbrake.io
[notice-v3]: https://docs.airbrake.io/docs/#create-notice-v3
[env_logger]: https://crates.io/crates/env_logger
[project-idkey]: https://s3.amazonaws.com/airbrake-github-assets/airbrake-ruby/project-id-key.png
[stderror]: https://doc.rust-lang.org/std/error
[json-object]: https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/enum.Json.html
