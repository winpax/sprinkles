//! Scoop shim helpers

use std::fmt::Display;

use quork::prelude::ListVariants;

use crate::packages::reference::package::Reference;

pub struct Shim {
    name: String,
    package: Reference,
}

impl Shim {
    /// A list of all possible shim file path extensions
    const SHIM_EXTENSIONS: [ShimExtension; 4] = ShimExtension::VARIANTS;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, quork::macros::ListVariants)]
/// All possible shim file path extensions
enum ShimExtension {
    /// Empty (no extension)
    Empty,
    /// .shim extension
    Shim,
    /// .cmd extension
    Cmd,
    /// .ps1 extension
    Ps1,
}

impl Display for ShimExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShimExtension::Empty => Ok(()),
            ShimExtension::Shim => Display::fmt(".shim", f),
            ShimExtension::Cmd => Display::fmt(".cmd", f),
            ShimExtension::Ps1 => Display::fmt(".ps1", f),
        }
    }
}
