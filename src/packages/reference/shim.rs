//! Helpers for local shims

use std::{fmt::Display, marker::PhantomData, path::PathBuf};

use crate::{contexts::ScoopContext, handles::shim::WeakShimHandle};

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
/// Known shim extensions
pub enum KnownExtension {
    /// .shim extension
    Shim,
    /// .ps1 extension
    Ps1,
    /// .cmd extension
    Cmd,
    /// .exe extension
    Exe,
}

impl KnownExtension {
    #[must_use]
    /// Get the extension as a string
    pub const fn as_str(&self) -> &'static str {
        match self {
            KnownExtension::Shim => ".shim",
            KnownExtension::Ps1 => ".ps1",
            KnownExtension::Cmd => ".cmd",
            KnownExtension::Exe => ".exe",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Shim extension type
pub struct ShimExtension(Option<KnownExtension>);

impl ShimExtension {
    /// .exe extension
    pub const EXE: Self = Self(Some(KnownExtension::Exe));
    /// .ps1 extension
    pub const PS1: Self = Self(Some(KnownExtension::Ps1));
    /// .cmd extension
    pub const CMD: Self = Self(Some(KnownExtension::Cmd));
    /// .shim extension
    pub const SHIM: Self = Self(Some(KnownExtension::Shim));
    /// Empty extension
    pub const EMPTY: Self = Self(None);

    #[must_use]
    /// Check if the shim is a binary
    pub fn is_binary(&self) -> bool {
        matches!(self.0, Some(KnownExtension::Exe))
    }

    #[must_use]
    /// Check if the shim is a text file
    pub fn is_text(&self) -> bool {
        !self.is_binary()
    }

    #[must_use]
    /// Check if the shim is a spec file
    ///
    /// This is a `.shim` file that specifies how to execute the program
    ///
    /// This is the most common type of shim, but is used in conjunction with a [`ShimExtension::Exe`] file
    pub fn is_spec(&self) -> bool {
        matches!(self.0, Some(KnownExtension::Shim))
    }

    #[must_use]
    /// Get the extension as a string
    pub const fn as_str(&self) -> &'static str {
        match self.0 {
            Some(known) => known.as_str(),
            None => "",
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
///
/// Note this is simply a reference to a shim.
/// To do any operations on the shim (i.e reading or writing), use a [`ShimHandle`] (See [`ShimReference::open_handle`]).
pub struct ShimReference<'a, C: ScoopContext> {
    name: &'a str,
    extension: ShimExtension,
    // Context is included here because the shim reference needs to reference a single shim
    // If it wasn't specific to a context there would be ambiguity as to which context the shim belongs
    ctx: PhantomData<C>,
}

// Manual implementation allows it to be copied even though
// `ScoopContext` is not `Copy`
impl<C: ScoopContext> Copy for ShimReference<'_, C> {}

impl<'a, C: ScoopContext> ShimReference<'a, C> {
    #[must_use]
    /// Get the extension this shim has
    pub fn extension(&self) -> ShimExtension {
        self.extension
    }

    #[must_use]
    /// Check if the shim is a binary
    pub fn is_binary(&self) -> bool {
        self.extension.is_binary()
    }

    #[must_use]
    /// Check if the shim is a text file
    pub fn is_text(&self) -> bool {
        self.extension.is_text()
    }

    #[must_use]
    /// Check if the shim is a spec file
    ///
    /// This is a `.shim` file that specifies how to execute the program
    ///
    /// This is the most common type of shim, but is used in conjunction with a [`ShimExtension::Exe`] file
    pub fn is_spec(&self) -> bool {
        self.extension.is_spec()
    }

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

    /// Open the shim handle if it exists
    pub fn open_handle<'c>(self, ctx: &'c C) -> Option<WeakShimHandle<'a, 'c, C>> {
        if self.exists(ctx).ok()? {
            Some(WeakShimHandle::new(self, ctx))
        } else {
            None
        }
    }
}
