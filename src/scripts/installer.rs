//! Utilities for running installers and uninstallers

use crate::packages::models::manifest::{Installer, Uninstaller};

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
/// Errors that can occur when running an installer
pub enum Error {}

#[allow(missing_docs)]
pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<Uninstaller> for Installer {
    fn from(uninstaller: Uninstaller) -> Self {
        Installer {
            file: uninstaller.file,
            comment: None,
            args: uninstaller.args,
            keep: Some(false),
            script: uninstaller.script,
        }
    }
}

#[must_use]
/// Controller for the execution of installers
pub struct Runner {
    installer: Installer,
}

impl Runner {
    /// Construct a new installer runner from an installer
    pub fn new(installer: impl Into<Installer>) -> Self {
        Self { installer: installer.into() }
    }

    /// Construct a new installer runner from an installer
    pub fn from_installer(installer: Installer) -> Self {
        Self::new(installer)
    }

    /// Construct a new installer runner from an uninstaller
    pub fn from_uninstaller(uninstaller: Uninstaller) -> Self {
        Self::new(uninstaller)
    }

    /// Run the installer
    ///
    /// # Errors
    /// - get fucked (todo)
    pub fn run(&self) -> Result<()> {
        todo!()
    }
}
