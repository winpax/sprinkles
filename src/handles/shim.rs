//! Shim handle helpers for interacting with shims stored locally on disk

pub mod spec;

use crate::{contexts::ScoopContext, packages::reference::shim::ShimReference};

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
#[derive(Debug, Clone, PartialEq, Eq)]
/// A shim handle
/// providing access to a shim stored locally on disk.
pub struct WeakShimHandle<'a, 'c, C: ScoopContext> {
    pub(self) shim: ShimReference<'a, C>,
    pub(self) ctx: &'c C,
}

// Manual implementation allows it to be copied even though
// `ScoopContext` is not `Copy`
impl<'a, 'c, C: ScoopContext> Copy for WeakShimHandle<'a, 'c, C> {}

impl<'a, 'c, C: ScoopContext> WeakShimHandle<'a, 'c, C> {
    #[inline]
    pub(crate) fn new(shim: ShimReference<'a, C>, ctx: &'c C) -> Self {
        Self { shim, ctx }
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

    /// Open the shim file
    ///
    /// # Errors
    /// Opening the file fails. See [`std::fs::File::open`] for more details
    fn open(&self) -> Result<std::fs::File> {
        std::fs::File::open(self.shim.path(self.ctx)).map_err(Error::OpeningShim)
    }

    #[must_use]
    /// Open the spec shim handle if this handle references a spec shim
    pub fn open_spec(self) -> Option<spec::ShimSpecHandle<'a, 'c, C>> {
        if !self.shim.is_spec() {
            return None;
        }

        spec::ShimSpecHandle::new(self).ok()
    }
}
