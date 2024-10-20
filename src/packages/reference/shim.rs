//! Scoop shim helpers
//!
//! Note that these provide helpers for referencing existing shims.
//!
//! For creating and modifying shims see [`crate::shim`]

use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use quork::prelude::ContainsTruth;
use regex::Regex;

use crate::contexts::ScoopContext;

use super::package;

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
/// Shimming errors
pub enum Error {
    #[error("Error removing shim: {0}")]
    RemovingShim(#[from] std::io::Error),
    #[error("Error renaming shim: {0}")]
    RegexError(#[from] regex::Error),
}

/// Result for shimming errors
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone, derive_more::Deref, derive_more::DerefMut)]
/// A list of references to shims locally on disk
pub struct ShimsReferences(Vec<ShimReference>);

impl ShimsReferences {
    /// Remove all shims if they exist
    ///
    /// # Errors
    /// - Error removing shim
    pub fn remove_all(&self, ctx: &impl ScoopContext) -> Result<()> {
        for shim in &self.0 {
            shim.remove(ctx)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A reference to a package's shim locally on disk
pub struct ShimReference {
    name: String,
    package: Option<package::Reference>,
}

impl ShimReference {
    /// A list of all possible shim file path extensions
    const SHIM_EXTENSIONS: [ShimExtension; 4] = [
        ShimExtension::Empty,
        ShimExtension::Shim,
        ShimExtension::Cmd,
        ShimExtension::Ps1,
    ];

    /// Check if the shim exists on disk
    pub fn exists() {
        unimplemented!()
    }

    /// Remove the given shim if it exists
    ///
    /// # Errors
    /// - Removing the shim fails
    pub fn remove(&self, ctx: &impl ScoopContext) -> Result<()> {
        for extension in Self::SHIM_EXTENSIONS {
            let path = self.path(ctx, extension);

            let alt_path = self.alt_path(ctx, extension);

            if let Some(alt_path) = alt_path {
                if alt_path.exists() {
                    std::fs::remove_file(alt_path)?;
                    return Ok(());
                }
            } else if path.exists() {
                std::fs::remove_file(path)?;

                let old_shims = Self::ls_old_shims(ctx)?;

                if old_shims.is_empty() && extension == ShimExtension::Shim {
                    let path = self.path(ctx, ShimExtension::Exe);
                    if path.exists() {
                        std::fs::remove_file(path)?;
                    }
                } else {
                    let latest_shim = unsafe {
                        old_shims
                            .iter()
                            .filter_map(|shim| Some((shim, shim.metadata().ok()?.modified().ok()?)))
                            .max_by_key(|(_, modified)| *modified)
                            .map(|(shim, _)| shim)
                            // Safety:
                            // This unwrap_unchecked call is safe  because we know that the vector is not empty
                            .unwrap_unchecked()
                    };

                    if let Some(new_name) = latest_shim.file_name().and_then(|name| name.to_str()) {
                        let patched_name = Regex::new(r".[^.]*$")?.replace(new_name, "");
                        std::fs::rename(latest_shim, PathBuf::from(patched_name.to_string()))?;
                    }
                }
            }
        }

        Ok(())
    }

    fn ls_old_shims(ctx: &impl ScoopContext) -> Result<Vec<PathBuf>> {
        let read_dir = std::fs::read_dir(ctx.shims_path())?;

        let old_shims = read_dir.filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            // I don't understand at all what this code does.
            // I have ported it almost verbatim from the original (https://github.com/ScoopInstaller/Scoop/blob/develop/lib/install.ps1#L209)
            if path
                .extension()
                .and_then(|extension| {
                    let disallowed_extensions: &[ShimExtension] = &Self::SHIM_EXTENSIONS[1..];

                    for search_extension in disallowed_extensions {
                        if *extension == *search_extension.to_string() {
                            return None;
                        }
                    }

                    Some(true)
                })
                .contains_truth()
            {
                Some(path)
            } else {
                None
            }
        });

        Ok(old_shims.collect())
    }

    fn path(&self, ctx: &impl ScoopContext, extension: ShimExtension) -> PathBuf {
        ctx.shims_path().join(format!("{}.{extension}", self.name))
    }

    fn alt_path(&self, ctx: &impl ScoopContext, extension: ShimExtension) -> Option<PathBuf> {
        let path = self.path(ctx, extension);

        self.package
            .as_ref()
            .and_then(package::Reference::name)
            .map(|app_name| {
                let mut alt_path = path;
                alt_path.extend(Path::new(&app_name));
                alt_path
            })
    }
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
    /// .exe extension
    Exe,
}

impl Display for ShimExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShimExtension::Empty => Ok(()),
            ShimExtension::Shim => Display::fmt(".shim", f),
            ShimExtension::Cmd => Display::fmt(".cmd", f),
            ShimExtension::Ps1 => Display::fmt(".ps1", f),
            ShimExtension::Exe => Display::fmt(".exe", f),
        }
    }
}
