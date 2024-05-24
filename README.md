# Sprinkles Library

[![Build & Test](https://github.com/winpax/sprinkles/actions/workflows/build.yml/badge.svg)](https://github.com/winpax/sprinkles/actions/workflows/build.yml)
[![Crates.io Version](https://img.shields.io/crates/v/sprinkles-rs)](https://crates.io/crates/sprinkles-rs)
[![docs.rs](https://img.shields.io/docsrs/sprinkles-rs)](https://docs.rs/sprinkles-rs)
[![Libraries.io dependency status for GitHub repo](https://img.shields.io/librariesio/github/winpax/sprinkles)](https://libraries.io/cargo/sprinkles-rs)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/sprinkles-rs)](https://crates.io/crates/sprinkles-rs)
[![Crates.io License](https://img.shields.io/crates/l/sprinkles-rs)](https://crates.io/crates/sprinkles-rs)
[![Crates.io MSRV](https://img.shields.io/crates/msrv/sprinkles-rs)](https://crates.io/crates/sprinkles-rs)

**Please note this library is currently in an early stage of development, and is not recommended for use in production.
There are likely to be breaking changes in the future, and there is no guarantee that the API will remain stable until v1.0.**

Sprinkles is a library for interacting with [Scoop](https://scoop.sh/), the Windows package manager.

It provides a high-level API for interacting with [Scoop](https://scoop.sh/), such as installing, updating, and removing packages.

## Example Usage

If you want a more in depth example of how to use the library, check out the [sfsu](https://github.com/winpax/sfsu) project.

```rust
use sprinkles::contexts::{User, ScoopContext};

let ctx = User::new();

let apps = ctx.installed_apps().unwrap();

println!("You have {} apps installed", apps.len());
```

## Supported Platforms

I will maintain support for the MSRV mentioned in Cargo.toml, although it may change across a major version.

Windows is the only supported platform at the moment, and this will likely not change, given that Scoop is only available on Windows.

**Made with 💗 by Juliette Cordor**
