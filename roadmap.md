# Development Roadmap

This document describes the current state of the project, focuses on the functions and features needed to reach parity with Airbrake clients in other languages. This should provide a better understanding of planned features, remaining work, timelines and milestones.

## Current Project State

The project supports the following features:

- Client proxy (without auth)
- Notifying via std::error::Error trait
- Notifying asyncronously via `notify`
- Notifying syncronously via `notify_sync`
- Notifier configuration
  - project_id
  - project_key
  - host
  - workers
  - proxy
  - app_version

## Considered Features

Comparing the features available in the Airbrake libraries for the Python, Ruby and Go projects, I believe the following features are required to reach feature parity with the other languages.

- Notify on panic
- `add_filter` middleware for Notify mutation and filtering
- `keys_blacklist` for obscuring sensitive information
- Notify supports [custom params](https://github.com/airbrake/pybrake#adding-custom-params)
- Proxy authentication
- Notify severity
- Performance Monitoring
  - Route Performance
  - Routes Breakdown
  - Database Query Stats

Along with these concrete features are some more abiguous developer oriented ergonomic changes:

- Public interfaces
- Builder pattern initialization
- Typed project id/key validation
- Client circuit breaker

## Branching Strategy

This repo will move forward with a partial [git flow](https://nvie.com/posts/a-successful-git-branching-model/#the-main-branches) branching strategy, reducing complexity in releasing future bug and security fixes.

Git flow, as described here typically has a `develop` and a `master` branch, though in our case there is little to be gained from a `master` as described in the document. I propose the following strategy:
- The existing `master` branch serve as the `develop` branch as described by Git Flow.
- Feature branches will branch to and from `master`.
- Release branch will be branched off `master` when appropriate, and not be merged back into any other branches.

## Timelines and Milestones

- Sprint 1: Ergonomic Changes
  - [x] Public interfaces
  - [x] Builder pattern
  - [x] Config validation
  - [x] More detailed error handling around sending notifications

- Sprint 2: Notifier client expansion
  - [ ] `add_filter`
  - [ ] `keys_blacklist`
  - [ ] Proxy authentication (not supported at all right now)
  - [x] ~~Circuit breaker~~ defer implementation to users

- Sprint 3: Notify features
  - [ ] Feature parity with [official API](https://docs.airbrake.io/docs/api/#create-notice-v3) (super close to done, just need to verify I haven't missed anything)
  - [x] Notify on panic (Technically done and working, but could use some additional cleanup)
  - [x] Support [custom parameters](https://github.com/airbrake/pybrake#adding-custom-params)
  - [x] Severity

- Pre-release work
  - [ ] Clean up documentation

- Possible 0.3.0 release candidate

- Update roadmap

- Sprint 4: Performance Monitoring
  - [ ] [Route Performance](https://docs.airbrake.io/docs/api/#route-performance-endpoint)
  - [ ] [Routes Breakdown](https://docs.airbrake.io/docs/api/#routes-breakdown-endpoint)
  - [ ] [Database Query Stats](https://docs.airbrake.io/docs/api/#database-query-stats)

- Possible 0.4.0 release candidate

- Evaluate further requisites for stable release
