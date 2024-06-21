# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking Changes

- Default logging directory for `User` context implementation is now `%LocalAppData%/sfsu/logs` instead of `<sfsu app path>/logs`
- Provide more default trait methods for `ScoopContext`. This may cause different outputs for certain implementations.
- Config type is now an associated type of `ScoopContext` rather than a generic type parameter
  - This saves a lot of boilerplate, internally and externally, having generics everywhere to account for the config type

## Changes

- Refactor substitutions trait. It now only requires `fn substitute` rather than `fn into_substituted`
- Do not do any manifest updates if the version is the same as the current version
- Replace `parallel` feature with `rayon` feature

## [0.15.2]

### Fixed

- Enable `tokio/fs` feature under `manifest-hashes` feature
- Fix cache path extension not being renamed by url segment

## [0.15.1]

### Fixed

- Cargo packaging
- Builds without default features

### Added

- Hashing algorithm benchmarks

For older version's changelogs, see the [releases](https://github.com/winpax/sprinkles/releases) page.

[Unreleased]: https://github.com/winpax/sfsu/compare/v0.15.1...HEAD
[0.15.2]: https://github.com/winpax/sprinkles/releases/tag/v0.15.2
[0.15.1]: https://github.com/winpax/sprinkles/releases/tag/v0.15.1
