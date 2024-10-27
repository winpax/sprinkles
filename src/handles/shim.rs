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

    pub fn open(&self, ctx: &C) -> Result<std::fs::File> {
        std::fs::File::open(self.shim.path(ctx)).map_err(Error::OpeningShim)
    }

    pub fn parse_spec(&self, ctx: &C) -> Option<Result<scoop_shim::Shim>> {
        if !self.shim.is_spec() {
            return None;
        }

        let file = self.open(ctx).ok()?;

        let spec = scoop_shim::Shim::from_reader(file).map_error(Error::ParsingSpec);

        Some(spec)
    }
}
