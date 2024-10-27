//! Shim handle helpers for interacting with shims stored locally on disk

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

    /// Open the shim file
    ///
    /// # Errors
    /// Opening the file fails. See [`std::fs::File::open`] for more details
    pub fn open(&self, ctx: &C) -> Result<std::fs::File> {
        std::fs::File::open(self.shim.path(ctx)).map_err(Error::OpeningShim)
    }

    /// Parse the spec file into a [`scoop_shim::Shim`]
    ///
    /// # Errors
    /// Parsing the spec file fails.
    /// Opening/reading from the spec file fails.
    ///
    /// See [`scoop_shim::Error`] for more details
    pub fn parse_spec(&self, ctx: &C) -> Option<Result<scoop_shim::Shim>> {
        if !self.shim.is_spec() {
            return None;
        }

        let mut file = self.open(ctx).ok()?;

        let spec = scoop_shim::from_reader(&mut file).map_err(Error::ParsingSpec);

        Some(spec)
    }

    /// Update the shim spec
    ///
    /// # Errors
    /// - Opening/reading from the spec file fails
    /// - Writing to the spec file fails
    pub fn update_spec(&self, ctx: &C, spec: &scoop_shim::Shim) -> Result<(), Error> {
        if !self.shim.is_spec() {
            return Err(Error::NonSpecUpdate);
        }

        let mut file = self.open(ctx)?;

        scoop_shim::to_writer(spec, &mut file).map_err(Error::ParsingSpec)?;

        Ok(())
    }
}
