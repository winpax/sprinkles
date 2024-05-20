# Sprinkles Library

[![Build & Test](https://github.com/winpax/sprinkles/actions/workflows/build.yml/badge.svg)](https://github.com/winpax/sprinkles/actions/workflows/build.yml)
![Crates.io Version](https://img.shields.io/crates/v/sprinkles-rs)
![docs.rs](https://img.shields.io/docsrs/sprinkles-rs)
![Libraries.io dependency status for GitHub repo](https://img.shields.io/librariesio/github/winpax/sprinkles)
![Crates.io Total Downloads](https://img.shields.io/crates/d/sprinkles-rs)
![Crates.io License](https://img.shields.io/crates/l/sprinkles-rs)
![Crates.io MSRV](https://img.shields.io/crates/msrv/sprinkles-rs)

Sprinkles is a library for interacting with Scoop, a Windows package manager.

It provides a high-level API for interacting with Scoop, such as installing, updating, and removing packages.

## Example Usage

If you want a more in depth example of how to use the library, check out the [sfsu](https://github.com/winpax/sfsu) project.

```rust
use sprinkles::contexts::{User, ScoopContext};

let ctx = User::new();

let apps = ctx.installed_apps().unwrap();

println!("You have {} apps installed", apps.len());
```

**Made with 💗 by Juliette Cordor**
