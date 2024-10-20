//! Helpers for local shims

use std::{fmt::Display, path::PathBuf};

use crate::contexts::ScoopContext;

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
/// Shimming errors
pub enum Error {
    #[error("Error removing shim: {0}")]
    RemovingShim(#[from] std::io::Error),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Shim extension type
pub enum ShimExtension {
    /// Empty (no extension)
    Empty,
    /// .shim extension
    Shim,
    /// .ps1 extension
    Ps1,
    /// .cmd extension
    Cmd,
    /// .exe extension
    Exe,
}

impl ShimExtension {
    #[must_use]
    /// Get the extension as a string
    pub const fn as_str(&self) -> &'static str {
        match self {
            ShimExtension::Empty => "",
            ShimExtension::Shim => ".shim",
            ShimExtension::Ps1 => ".ps1",
            ShimExtension::Cmd => ".cmd",
            ShimExtension::Exe => ".exe",
        }
    }
}

impl Display for ShimExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A reference to a package's shim locally on disk
pub struct ShimReference {
    name: String,
    extension: ShimExtension,
}

impl ShimReference {
    /// Check if the shim exists on disk
    ///
    /// # Errors
    /// - Checking the existence of the shim fails (see [`std::fs::exists`] for more details)
    pub fn exists(&self, ctx: &impl ScoopContext) -> Result<bool, Error> {
        Ok(self.path(ctx).try_exists()?)
    }

    /// Get the full path to the shim
    pub fn path(&self, ctx: &impl ScoopContext) -> PathBuf {
        ctx.shims_path()
            .join(format!("{}.{}", self.name, self.extension.as_str()))
    }
}
