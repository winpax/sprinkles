//! Shim handle helpers for interacting with shims stored locally on disk

use std::ops::Deref;

use crate::{
    contexts::{self, ScoopContext},
    packages::reference::shim::ShimReference,
};

use super::ShimHandle;

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
/// Shim handling errors
pub enum Error {
    #[error("{0}")]
    RemovingShim(std::io::Error),
    #[error("{0}")]
    OpeningShim(std::io::Error),
    #[error("Error parsing shim spec: {0}")]
    ParsingSpec(scoop_shim::Error),

    #[error("Could not update shim spec. ShimHandle does not handle a spec file")]
    NonSpecUpdate,
}

/// Shim result type
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[must_use]
#[derive(Debug, PartialEq, Eq)]
/// A shim handle
/// providing access to a shim stored locally on disk.
pub struct ShimSpecHandle<'a, 'c, C: ScoopContext> {
    handle: ShimHandle<'a, 'c, C>,
}

impl<'a, 'c, C: ScoopContext> ShimSpecHandle<'a, 'c, C> {
    #[inline]
    pub(crate) fn new(handle: ShimHandle<'a, 'c, C>) -> Self {
        Self { handle }
    }

    /// Remove the shim if it exists
    ///
    /// # Errors
    /// Removing the shim fails. See [`std::fs::remove_file`] for more details
    pub fn remove(&self) -> Result<()> {
        std::fs::remove_file(self.shim.path(self.ctx)).map_err(Error::RemovingShim)
    }

    #[must_use]
    /// Get the reference that this [`ShimHandle`] was created from
    pub fn reference(&self) -> &ShimReference<'a, C> {
        &self.shim
    }

    #[inline]
    /// Restore the weak handle with no information about the type of shim
    pub fn weak_handle(self) -> super::ShimHandle<'a, 'c, C> {
        self.handle
    }
}

impl<'a, 'c, C: contexts::ScoopContext> Deref for ShimSpecHandle<'a, 'c, C> {
    type Target = ShimHandle<'a, 'c, C>;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}
