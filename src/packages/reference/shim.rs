//! Helpers for local shims

use std::{fmt::Display, marker::PhantomData, path::PathBuf};

use crate::contexts::ScoopContext;

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
/// Shimming errors
pub enum Error {
    #[error("Non-specific IO error: {0}")]
    GeneralIO(#[from] std::io::Error),
    #[error("Error removing shim: {0}")]
    RemovingShim(std::io::Error),
    #[error("Error checking shim existence: {0}")]
    CheckingExistence(std::io::Error),
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// A reference to a package's shim locally on disk
pub struct ShimReference<'a, C: ScoopContext> {
    name: &'a str,
    extension: ShimExtension,
    // Context is included here because the shim reference needs to reference a single shim
    // If it wasn't specific to a context there would be ambiguity as to which context the shim belongs
    ctx: PhantomData<C>,
}

impl<'a, C: ScoopContext> ShimReference<'a, C> {
    /// Check if the shim exists on disk
    ///
    /// # Errors
    /// Checking the existence of the shim fails. See [`std::fs::exists`] for more details)
    pub fn exists(&self, ctx: &C) -> Result<bool, Error> {
        self.path(ctx)
            .try_exists()
            .map_err(Error::CheckingExistence)
    }

    #[must_use]
    /// Get the full path to the shim
    pub fn path(&self, ctx: &C) -> PathBuf {
        ctx.shims_path()
            .join(format!("{}.{}", self.name, self.extension.as_str()))
    }

    /// Remove the shim if it exists
    ///
    /// # Errors
    /// Removing the shim fails. See [`std::fs::remove_file`] for more details
    pub fn remove(&self, ctx: &C) -> Result<(), Error> {
        std::fs::remove_file(self.path(ctx)).map_err(Error::RemovingShim)
    }
}
