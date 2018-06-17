Airbrake Rust
=============

[![Build Status](https://travis-ci.org/kyrylo/airbrake-rust.svg?branch=master)](https://travis-ci.org/kyrylo/airbrake-rust)
[![Crates.io](https://img.shields.io/crates/v/airbrake.svg)](https://crates.io/crates/airbrake)


* [Documentation](https://docs.rs/crate/airbrake)
* [Airbrake Rust README](https://github.com/kyrylo/airbrake-rust)

Introduction
------------

_Airbrake Rust_ is an [Airbrake][airbrake.io] notifier library for the Rust
Programming language. The library provides minimalist API that enables the
ability to send Rust errors to the Airbrake dashboard.

Key features
------------

* Backtrace support

Installation
------------

### Cargo

Add the crate to your Cargo.toml:

```toml
[dependencies]
airbrake = "0.3"
```

Examples
--------

### Basic example

This is the minimal example that you can use to test Airbrake Rust with your
project.

```rust
extern crate airbrake;

let notifier = airbrake::Notifier::new(airbrake::Config {
    project_id: 113743,
    project_key: String::from("81bbff95d52f8856c770bb39e827f3f6"),
    ..Default::default()
});

// Get an error.
let error = "xc".parse::<u32>().err().unwrap();

// Build a notice from the error.
let notice = notifier.build_notice(error);

// Send the notice to Airbrake.
notifier.notify(notice);
```

Configuration
-------------

### project_id && project_key

You **must** set both `project_id` & `project_key`.

To find your `project_id` and `project_key` navigate to your project's _General
Settings_ and copy the values from the right sidebar.

![][project-idkey]

```rust
airbrake::Config {
    project_id: 1,
    project_key: "key",
    ..Default::default()
};
```

### proxy_url

If your server is not able to directly reach Airbrake, you can route your errors
through a proxy. By default, Airbrake Rust uses a direct connection.

```rust
airbrake::Config {
    proxy_url: Some(String::from("http://localhost:8080")),
    ..Default::default()
};
```

[airbrake.io]: https://airbrake.io
[project-idkey]: https://s3.amazonaws.com/airbrake-github-assets/airbrake-ruby/project-id-key.png
