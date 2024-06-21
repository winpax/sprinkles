//! Installer helpers

use std::{future::IntoFuture, process::Output};

use futures::FutureExt;

use crate::{config, contexts::ScoopContext, packages::manifest::Installer, scripts};

use super::models::manifest::{InstallerRunner, TOrArrayOfTs};

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
/// Errors that can occur when running an installer
pub enum Error {
    #[error("Installer I/O error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Installer script error: {0}")]
    Scripts(#[from] scripts::Error),
}

/// Installer result type
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
/// An installer host
///
/// This is used to run the installers
pub struct InstallerHost<'ctx, C: ScoopContext<config::Scoop>> {
    ctx: &'ctx C,
    installer: Installer,
    runner: InstallerRunner,
}

impl<'ctx, C: ScoopContext<config::Scoop>> InstallerHost<'ctx, C> {
    /// Create a new installer host
    pub fn new(ctx: &'ctx C, installer: Installer, runner: InstallerRunner) -> Self {
        Self {
            ctx,
            installer,
            runner,
        }
    }

    #[must_use]
    /// Create a new installer host from an installer
    pub fn from_installer(ctx: &'ctx C, installer: Installer) -> Option<Self> {
        let runner = installer.get_runner()?;
        Some(Self::new(ctx, installer, runner))
    }

    /// Run the installer
    ///
    /// # Errors
    /// - If the installer could not be run
    pub async fn run(self) -> Result<Output> {
        let runner = self.runner;
        let args = self.installer.args.clone().map(TOrArrayOfTs::to_vec);

        let output = match runner {
            InstallerRunner::File(file) => {
                let mut command = std::process::Command::new(file);

                if let Some(ref args) = args {
                    command.args(args)
                } else {
                    &mut command
                }
                .spawn()?
                .wait_with_output()?
            }
            InstallerRunner::Script(script) => script.save(self.ctx)?.run().await?,
        };

        Ok(output)
    }
}

impl<'ctx, C: ScoopContext<config::Scoop>> IntoFuture for InstallerHost<'ctx, C> {
    type Output = Result<Output>;

    type IntoFuture =
        std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'ctx>>;

    fn into_future(self) -> Self::IntoFuture {
        self.run().boxed()
    }
}

impl Installer {
    #[must_use]
    /// Get the installer runner
    pub fn get_runner(&self) -> Option<InstallerRunner> {
        self.script
            .clone()
            .map(InstallerRunner::Script)
            .or_else(|| self.file.clone().map(InstallerRunner::File))
    }

    /// Get the installer host for the installer
    ///
    /// Will return `None` if the installer does not have a script or file
    pub fn host<C: ScoopContext<config::Scoop>>(self, ctx: &C) -> Option<InstallerHost<'_, C>> {
        InstallerHost::from_installer(ctx, self)
    }
}

#[cfg(test)]
mod tests {
    use scripts::PowershellScript;

    use crate::contexts::User;

    use super::*;

    #[tokio::test]
    async fn test_powershell_hello_world() {
        let ctx = User::new();

        let installer = Installer {
            comment: None,
            args: None,
            file: None,
            keep: None,
            script: Some(PowershellScript::new("Write-Host 'Hello, world!'")),
        };

        let host = installer.host(&ctx).unwrap();

        let output = host.await.unwrap();

        assert_eq!(output.status.code(), Some(0));
        assert_eq!(output.stdout, b"Hello, world!\r\n");
    }
}
