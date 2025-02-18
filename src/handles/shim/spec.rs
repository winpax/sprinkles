//! Shim handle helpers for interacting with shims stored locally on disk

use std::ops::Deref;

use crate::{
    contexts::{self, ScoopContext},
    packages::reference::shim::ShimReference,
};

use super::{Error, Result, WeakShimHandle};

#[must_use]
#[derive(Debug)]
/// A shim handle
/// providing access to a shim stored locally on disk.
pub struct ShimSpecHandle<'c, C: ScoopContext> {
    handle: WeakShimHandle<'c, C>,
    spec: scoop_shim::Shim,
}

impl<'c, C: ScoopContext> ShimSpecHandle<'c, C> {
    #[inline]
    pub(crate) fn new(handle: WeakShimHandle<'c, C>) -> Result<Self> {
        let spec = Self::parse_spec(&handle)?;

        Ok(Self { handle, spec })
    }

    /// Remove the shim if it exists
    ///
    /// # Errors
    /// Removing the shim fails. See [`std::fs::remove_file`] for more details
    pub fn remove(&self) -> Result<()> {
        std::fs::remove_file(self.handle.shim.path(self.handle.ctx)).map_err(Error::RemovingShim)
    }

    #[must_use]
    /// Get the reference that this [`ShimHandle`] was created from
    pub fn reference(&self) -> &ShimReference<C> {
        self.handle.reference()
    }

    #[inline]
    /// Restore the weak handle with no information about the type of shim
    pub fn weak_handle(self) -> super::WeakShimHandle<'c, C> {
        self.handle
    }

    /// Parse the spec file into a [`scoop_shim::Shim`]
    ///
    /// # Errors
    /// Parsing the spec file fails.
    /// Opening/reading from the spec file fails.
    ///
    /// See [`scoop_shim::Error`] for more details
    fn parse_spec(handle: &WeakShimHandle<'c, C>) -> Result<scoop_shim::Shim> {
        let mut file = handle.open()?;

        scoop_shim::from_reader(&mut file).map_err(Error::ParsingSpec)
    }

    /// Save the shim spec to the shim file
    ///
    /// # Errors
    /// - Opening/reading from the spec file fails
    /// - Writing to the spec file fails
    pub fn save_spec(&self, spec: &scoop_shim::Shim) -> Result<(), Error> {
        if !self.handle.shim.is_spec() {
            return Err(Error::NonSpecUpdate);
        }

        let mut file = self.handle.open()?;

        scoop_shim::to_writer(spec, &mut file).map_err(Error::ParsingSpec)?;

        Ok(())
    }

    #[must_use]
    /// Get a reference to the Scoop shim spec
    pub fn spec(&self) -> &scoop_shim::Shim {
        &self.spec
    }

    #[must_use]
    /// Get a mutable reference to the Scoop shim spec
    pub fn spec_mut(&mut self) -> &mut scoop_shim::Shim {
        &mut self.spec
    }
}

impl<'c, C: contexts::ScoopContext> Deref for ShimSpecHandle<'c, C> {
    type Target = WeakShimHandle<'c, C>;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}
