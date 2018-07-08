Airbrake Rust
=============

[![Build Status](https://travis-ci.org/kyrylo/airbrake-rust.svg?branch=master)][travis]
[![Crates.io](https://img.shields.io/crates/v/airbrake.svg)][crate]

* [Documentation][crate-docs]
* [Airbrake Rust README][github-readme]

Introduction
------------

_Airbrake Rust_ is an [Airbrake][airbrake.io] notifier library for the Rust
Programming language. The library provides minimalist API that enables the
ability to send Rust errors to the Airbrake dashboard.

Key features
------------

* Backtrace support
* Proxy support

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

// Initialize an Airbrake notifier (client).
let notifier = airbrake::Notifier::new(airbrake::Config {
    project_id: 113743,
    project_key: "81bbff95d52f8856c770bb39e827f3f6",
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

If your server is not able to reach Airbrake directly, then you can route your
errors through a proxy. By default, Airbrake Rust uses direct connection. When
`proxy_url` is provided, then it sends errors to that proxy.

```rust
airbrake::Config {
    proxy_url: "http://localhost:8080",
    ..Default::default()
};
```

### app_version

The version of your application that you can pass to differentiate exceptions
between multiple versions. It's not set by default.

```rust
airbrake::Config {
    app_version: "v1.2.3",
    ..Default::default()
};
```

API
---

### airbrake::Notifier

#### new

Creates a new Airbrake notifier. Accepts a `notifier::Config`.

```rust
let notifier = airbrake::Notifier::new(airbrake::Config {
    // ...
});
```

#### build_notice

Builds an `airbrake::Notice` from the given error. The notice carries error type
and error message.

```rust
let notice = notifier.build_notice(error);
```

#### notify

Sends the notice object to Airbrake.

```rust
notifier.notify(notice);
```

### Notice

#### set_backtrace

Attaches a backtrace provided by the [`backtrace`][backtrace] library.

```rust
extern crate backtrace;

let backtrace = Backtrace::new();
notice.set_backtrace(backtrace);
```

#### set_params

Attaches arbitrary string parameters.

```rust
let mut params = std::collections::HashMap::new();
params.insert(String::from("mango"), airbrake::Param::Int32(42));
params.insert(
    String::from("banana"),
    airbrake::Param::String(String::from("tasty")),
);

notice.set_params(params);
```

#### set_app_version

Sets app version.

```rust
notice.set_app_version("v1.2.3");
```

Additional notes
----------------

### Exception limit (not implemented yet)

The maximum size of an `airbrake::Notice` is 64KB (in its JSON form). Exceptions
that exceed this limit will be truncated to fit the size.

### Library status

This is not an official library (yet) despite the fact that I work for
Airbrake. The library is in development, so please use with caution.

Platform support
----------------

TBA

Contact
-------

In case you have a problem, question or a bug report, feel free to:

* [file an issue][issues]
* [send a PR][pulls]
* [send me an email][email]

Licence
-------

The project uses the MIT License. See LICENCE.md for details.

[airbrake.io]: https://airbrake.io
[crate]: https://crates.io/crates/airbrake
[travis]: https://travis-ci.org/kyrylo/airbrake-rust
[crate-docs]: https://docs.rs/airbrake
[project-idkey]: https://s3.amazonaws.com/airbrake-github-assets/airbrake-ruby/project-id-key.png
[backtrace]: https://docs.rs/backtrace
[issues]: https://github.com/kyrylo/airbrake-rust/issues
[pulls]: https://github.com/kyrylo/airbrake-rust/pulls
[email]: mailto:silin@kyrylo.org
[github-readme]: https://github.com/kyrylo/airbrake-rust
