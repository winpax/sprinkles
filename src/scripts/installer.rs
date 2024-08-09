//! Utilities for running installers and uninstallers

use crate::contexts::ScoopContext;
use crate::handles::packages::PackageHandle;
use crate::hash::substitutions::{Substitute, SubstitutionMap};
use crate::hash::url_ext::UrlExt;
use crate::{
    packages::models::manifest::{Installer, Uninstaller},
    packages::Manifest,
    Architecture,
};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
/// Errors that can occur when running an installer
pub enum Error {
    #[error("Invalid installer. No file name or urls were provided")]
    MissingFileName,
    #[error("The uninstall program is not located in the version directory")]
    ProgramOutsideVersionDir,
    #[error("The uninstall program could not be found")]
    ProgramNotFound,
    #[error("Invalid url provided in manifest: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Could not open the package handle: {0}")]
    HandleError(#[from] crate::handles::packages::Error),
    #[error("Could not run the powershell script: {0}")]
    PowershellError(#[from] super::Error),
}

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
pub struct Runner<'a, 'c, C> {
    handle: &'a PackageHandle<'c, C>,
    installer: Installer,
    architecture: Architecture,
}

impl<'a, 'c, C: ScoopContext> Runner<'a, 'c, C> {
    /// Construct a new installer runner from an installer
    pub fn new(handle: &'a PackageHandle<'c, C>, installer: impl Into<Installer>) -> Self {
        Self {
            handle,
            installer: installer.into(),
            architecture: Architecture::ARCH,
        }
    }

    /// Provide a specific architecture to the installer runner
    pub fn with_architecture(self, architecture: Architecture) -> Self {
        Self {
            architecture,
            ..self
        }
    }

    /// Run the installer
    ///
    /// # Errors
    /// - get fucked (todo)
    pub fn run(self, ctx: &impl ScoopContext, manifest: &Manifest) -> Result<()> {
        let installer = self.installer;

        if installer.file.is_some() || installer.args.is_some() {
            let name = if let Some(name) = installer.file {
                name
            } else {
                let install_config = manifest.install_config(self.architecture);

                if let Some(urls) = install_config.url {
                    let mut urls = urls.iter();
                    let first_url = urls.next();

                    if let Some(first_url) = first_url {
                        Url::parse(first_url)?.remote_filename()
                    } else {
                        return Err(Error::MissingFileName);
                    }
                } else {
                    return Err(Error::MissingFileName);
                }
            };

            let version_dir = self.handle.version_dir();
            let prog_name = version_dir.join(name);

            if !prog_name.starts_with(&version_dir) {
                return Err(Error::ProgramOutsideVersionDir);
            } else if !prog_name.exists() {
                return Err(Error::ProgramNotFound);
            }

            let substitutions = {
                let mut map = HashMap::new();
                map.insert("$dir", version_dir.to_string_lossy().to_string());
                map.insert("$global", (C::CONTEXT_NAME == "global").to_string());
                map.insert("$version", manifest.version.to_string());

                SubstitutionMap::from(map)
            };

            let args = installer
                .args
                .map(|args| args.into_substituted(&substitutions, false))
                .map(|args| args.to_vec())
                .unwrap_or_default();

            if prog_name.extension() == Some(std::ffi::OsStr::new("ps1")) {
                let script = super::PowershellScript::from_path(prog_name)?;
                let mut runner = script.save(ctx)?;
                runner.set_args(args);
                runner.run()?;
            } else {}
        }

        todo!()
    }
}
