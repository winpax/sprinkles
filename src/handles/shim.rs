use crate::{contexts::ScoopContext, packages::reference::shim::ShimReference};

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
/// Shim handling errors
pub enum Error {
    #[error("{0}")]
    RemovingShim(std::io::Error),
}

/// Shim result type
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[must_use]
#[derive(Debug, PartialEq, Eq)]
/// A shim handle
/// providing access to a shim stored locally on disk.
pub struct ShimHandle<'a, C: ScoopContext> {
    pub(self) shim: ShimReference<'a, C>,
}

impl<'a, C: ScoopContext> ShimHandle<'a, C> {
    pub(crate) fn new(shim: ShimReference<'a, C>) -> Self {
        Self { shim }
    }

    /// Remove the shim if it exists
    ///
    /// # Errors
    /// Removing the shim fails. See [`std::fs::remove_file`] for more details
    pub fn remove(&self, ctx: &C) -> Result<()> {
        std::fs::remove_file(self.shim.path(ctx)).map_err(Error::RemovingShim)
    }

    #[must_use]
    /// Get the reference that this [`ShimHandle`] was created from
    pub fn reference(&self) -> &ShimReference<'a, C> {
        &self.shim
    }
}
