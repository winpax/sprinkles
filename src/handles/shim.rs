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
#[derive(Debug, PartialEq, Eq)]
/// A shim handle
/// providing access to a shim stored locally on disk.
pub struct ShimHandle<'a, 'c, C: ScoopContext> {
    pub(self) shim: ShimReference<'a, C>,
    pub(self) ctx: &'c C,
}

impl<'a, 'c, C: ScoopContext> ShimHandle<'a, 'c, C> {
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

        Some(spec::ShimSpecHandle::new(self))
    }

    /// Parse the spec file into a [`scoop_shim::Shim`]
    ///
    /// # Errors
    /// Parsing the spec file fails.
    /// Opening/reading from the spec file fails.
    ///
    /// See [`scoop_shim::Error`] for more details
    pub fn parse_spec(&self) -> Option<Result<scoop_shim::Shim>> {
        if !self.shim.is_spec() {
            return None;
        }

        let mut file = self.open().ok()?;

        let spec = scoop_shim::from_reader(&mut file).map_err(Error::ParsingSpec);

        Some(spec)
    }

    /// Save the shim spec to the shim file
    ///
    /// # Errors
    /// - Opening/reading from the spec file fails
    /// - Writing to the spec file fails
    pub fn save_spec(&self, spec: &scoop_shim::Shim) -> Result<(), Error> {
        if !self.shim.is_spec() {
            return Err(Error::NonSpecUpdate);
        }

        let mut file = self.open()?;

        scoop_shim::to_writer(spec, &mut file).map_err(Error::ParsingSpec)?;

        Ok(())
    }
}
