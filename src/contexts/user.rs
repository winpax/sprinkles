use std::path::{Path, PathBuf};

use crate::{config, contexts::Error, git, system::paths::WindowsPath};

#[derive(Debug, Clone)]
/// User's Scoop install adapter
pub struct User {
    config: config::Scoop,
    path: PathBuf,
}

impl User {
    #[must_use]
    /// Construct a new user context adapter
    pub fn new() -> Self {
        let path = {
            if let Some(path) = crate::env::paths::scoop_path() {
                path
            } else if let Ok(path) = Ok::<_, ()>(
                config::Scoop::load()
                    .map(|config| config.root_path)
                    .unwrap(),
            ) {
                path
            } else {
                directories::BaseDirs::new()
                    .expect("user directories")
                    .home_dir()
                    .join("scoop")
            }
        };

        let path = if path.exists() {
            dunce::canonicalize(path).expect("failed to find real path to scoop")
        } else {
            panic!("Scoop path does not exist");
        };

        let config = config::Scoop::load().expect("scoop config loaded correctly");

        Self { config, path }
    }
}

impl Default for User {
    fn default() -> Self {
        Self::new()
    }
}

impl super::ScoopContext for User {
    type Config = config::Scoop;

    const APP_NAME: &'static str = "scoop";
    const CONTEXT_NAME: &'static str = "user";

    /// Load the Scoop configuration
    ///
    /// # Errors
    /// - Could not load the configuration
    fn config(&self) -> &config::Scoop {
        &self.config
    }

    fn config_mut(&mut self) -> &mut config::Scoop {
        &mut self.config
    }

    fn symlinks_enabled(&self) -> bool {
        !self.config.no_junction
    }

    fn proxy(&self) -> Option<&crate::proxy::Proxy> {
        self.config().proxy.as_ref()
    }

    #[must_use]
    /// Gets the user's scoop path, via either the default path or as provided by the SCOOP env variable
    ///
    /// Will ignore the global scoop path
    ///
    /// # Panics
    /// - There is no home folder
    /// - The discovered scoop path does not exist
    fn path(&self) -> &Path {
        &self.path
    }

    #[must_use]
    /// Gets the user's scoop cache path
    fn cache_path(&self) -> PathBuf {
        if let Some(cache_path) = crate::env::paths::scoop_cache() {
            cache_path
        } else if let Some(cache_path) = self.config().cache_path.as_ref() {
            cache_path.clone()
        } else {
            self.sub_path("cache")
        }
    }

    /// Get the path to the log directory
    ///
    /// By default, this will be the user's "%LocalAppData%/sfsu/logs" directory,
    /// or, in the case of a debug build, "<current working directory>/logs".
    ///
    /// # Errors
    /// - Creating the directory fails
    fn logging_dir(&self) -> std::io::Result<PathBuf> {
        #[cfg(not(debug_assertions))]
        let logs_path = self.apps_path().join("sfsu").join("current").join("logs");

        #[cfg(debug_assertions)]
        let logs_path: PathBuf = WindowsPath::LocalAppData
            .into_path()
            .or_else(|| std::env::var("LocalAppData").ok().map(Into::into))
            .expect("either windows defined local app data or env var `LocalAppData`")
            .join("sfsu")
            .join("logs");

        if !logs_path.exists() {
            std::fs::create_dir_all(&logs_path)?;
        }

        Ok(logs_path)
    }

    /// Open Scoop app repository
    ///
    /// # Errors
    /// - The Scoop app could not be opened as a repository
    fn open_repo(&self) -> Option<git::Result<git::Repo>> {
        Some(git::Repo::scoop_app(self))
    }

    /// Check if Scoop is outdated
    ///
    /// # Errors
    /// - The Scoop app could not be opened as a repository
    /// - The Scoop app could not be checked for updates
    async fn outdated(&self) -> Result<bool, Error> {
        let config = self.config();
        let scoop_repo = self.open_repo().expect("scoop repo")?;

        let current_branch = scoop_repo.current_branch()?;
        let scoop_config_branch = config.scoop_branch.name();

        if current_branch != scoop_config_branch {
            scoop_repo.checkout(scoop_config_branch)?;
            debug!("Switched to branch {}", scoop_config_branch);
            return Ok(true);
        }

        Ok(scoop_repo.outdated()?)
    }
}
