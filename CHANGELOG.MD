<!-- markdownlint-disable MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.18.0]

### Added

- Better support for compiling on non-windows platforms

### Changed

- `leaf` function now returns a `&str` instead of a `String`
- Added optional name parameter to `DownloadHandle::new`

## [0.17.0]

### Fixed

- Fix `XDG_CONFIG_HOME` type

### Changed

- Minor updates for `gix` monorepo breaking changes
- Updated non-windows error message
- Dependency updates

### Breaking Changes

- Updated MSRV

## [0.16.0]

### Breaking Changes

- Default logging directory for `User` context implementation is now `%LocalAppData%/sfsu/logs` instead
  of `<sfsu app path>/logs`
- Provide more default trait methods for `ScoopContext`. This may cause different outputs for certain implementations.
- Config type is now an associated type of `ScoopContext` rather than a generic type parameter
  - This saves a lot of boilerplate, internally and externally, having generics everywhere to account for the config
    type
- Removed `v1` feature. `v1` breaking changes have been merged in this release.
- Updated git2 to v0.19
- Renamed `AliasArray` to `NestedArray`
- Renamed `TOrArrayOfTs` to `SingleOrArray`
- Renamed `Downloader` to `DownloadHandle`
- Removed `Default` implementations for `User`
- Added fallible `Result` return for `User` and `Global`
- If Scoop path cannot be found both `User` and `Global` will return an error
  - Previously `User` would panic and `Global` would return the invalid path anyway
- Removed `Uninstaller` struct
- `Uninstaller` struct has been replaced with `Installer` in the manifest
- Installer host `run` function will now remove the installer file if `keep` is not set to true
- `CreateManifest` `from_str` now takes `impl AsRef<str>` rather than `String`
- An empty hash string now deserializes as `None` rather than throwing an error
- Feature `rustls-tls` has been removed
- Feature `rustls-tls-native-roots` has been removed
- Feature `vendored-libgit2` has been removed
- Feature `vendored-openssl` has been removed
- Feature `zlib-ng` has been removed
- Feature `zlib-ng-compat` has been removed
- Feature `zlib-stock` has been removed

### Changes

- Refactor substitutions trait. It now only requires `fn substitute` rather than `fn into_substituted`
- Do not do any manifest updates if the version is the same as the current version
- Replace `parallel` feature with `rayon` feature
- Updated `derive_more` to v1.0
- Performance improvements for `Process::find_running` when using `Process::ExactExe`
- Updated quick-xml to v0.34
- Builds will fail less often by falling back to
  the default buckets if the buckets.json file fails to download

### Added

- Added `EmptyConfig` struct for when your implementation does not have a config
- `InstallerHost` struct for running installers
- Tests for `InstallerHost` and `PowershellScript`
- `from/to_object` methods for Scoop config for converting to/from serde_json objects
- Implement `FromStr` for `ScoopBranch`
- `array` module for working with nested arrays
- Implement `DoubleEndedIterator`, `FusedIterator` and `ExactSizeIterator` for `NestedIterator`
- Sha512 hash benchmark
- Added running function for package handles
- Function for finding running processes
- Process enum for handling the finding of running processes
- Implemented `Manifest` to `PackageHandle` conversion
- Downloader trait for downloading files
- Added Summary struct for determining what will run for (un)installers and scripts
- Implemented `Runner` for (un)installers
- `VersionHandle` struct for handling version directories
- Implemented `list_versions` for `PackageHandle`
- Implemented `substitute` for `InstallerHost`
- Made substitution module public

### Removed

- Removed bench files and file benchmarks
- Removed blake3 benchmarks

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

[Unreleased]: https://github.com/winpax/sprinkles/compare/v0.18.0...HEAD
[0.18.0]: https://github.com/winpax/sprinkles/releases/tag/v0.18.0
[0.17.0]: https://github.com/winpax/sprinkles/releases/tag/v0.17.0
[0.16.0]: https://github.com/winpax/sprinkles/releases/tag/v0.16.0
[0.15.2]: https://github.com/winpax/sprinkles/releases/tag/v0.15.2
[0.15.1]: https://github.com/winpax/sprinkles/releases/tag/v0.15.1
