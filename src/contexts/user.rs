use std::{
    fs::File,
    path::{Path, PathBuf},
};

use chrono::Local;

use crate::{config, contexts::Error, git};

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

impl super::ScoopContext<config::Scoop> for User {
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
    /// Gets the user's scoop apps path
    fn apps_path(&self) -> PathBuf {
        self.sub_path("apps")
    }

    #[must_use]
    /// Gets the user's scoop buckets path
    fn buckets_path(&self) -> PathBuf {
        self.sub_path("buckets")
    }

    #[must_use]
    /// Gets the user's scoop cache path
    fn cache_path(&self) -> PathBuf {
        if let Some(cache_path) = crate::env::paths::scoop_cache() {
            cache_path
        } else if let Some(cache_path) = config::Scoop::load()
            .ok()
            .and_then(|config| config.cache_path)
        {
            cache_path
        } else {
            self.sub_path("cache")
        }
    }

    #[must_use]
    /// Gets the user's scoop persist path
    fn persist_path(&self) -> PathBuf {
        self.sub_path("persist")
    }

    #[must_use]
    /// Gets the user's scoop shims path
    fn shims_path(&self) -> PathBuf {
        self.sub_path("shims")
    }

    #[must_use]
    /// Gets the user's scoop workspace path
    fn workspace_path(&self) -> PathBuf {
        self.sub_path("workspace")
    }

    /// Get the path to the log directory
    ///
    /// # Errors
    /// - Creating the directory fails
    fn logging_dir(&self) -> std::io::Result<PathBuf> {
        #[cfg(not(debug_assertions))]
        let logs_path = self.apps_path().join("sfsu").join("current").join("logs");

        #[cfg(debug_assertions)]
        let logs_path = std::env::current_dir()?.join("logs");

        if !logs_path.exists() {
            std::fs::create_dir_all(&logs_path)?;
        }

        Ok(logs_path)
    }

    /// Create a new log file
    ///
    /// # Errors
    /// - Creating the file fails
    ///
    /// # Panics
    /// - Could not convert tokio file into std file
    async fn new_log(&self) -> Result<File, Error> {
        let logs_dir = self.logging_dir()?;
        let date = Local::now();

        let log_file = async {
            use tokio::fs::File;

            let mut i = 0;
            loop {
                i += 1;

                let log_path =
                    logs_dir.join(format!("sfsu-{}-{i}.log", date.format("%Y-%m-%d-%H-%M-%S")));

                if !log_path.exists() {
                    break File::create(log_path).await;
                }
            }
        };
        let timeout = async {
            use std::time::Duration;
            use tokio::time;

            time::sleep(Duration::from_secs(5)).await;
        };

        let log_file = tokio::select! {
            res = log_file => Ok(res),
            () = timeout => Err(Error::TimeoutCreatingLog),
        }??;

        Ok(log_file
            .try_into_std()
            .expect("converted tokio file into std file"))
    }

    /// Create a new log file
    ///
    /// This function is synchronous and does not allow for timeouts.
    /// If for some reason there are no available log files, this function will block indefinitely.
    ///
    /// # Errors
    /// - Creating the file fails
    fn new_log_sync(&self) -> Result<File, Error> {
        let logs_dir = self.logging_dir()?;
        let date = Local::now();

        let mut i = 0;
        let file = loop {
            i += 1;

            let log_path =
                logs_dir.join(format!("sfsu-{}-{i}.log", date.format("%Y-%m-%d-%H-%M-%S")));

            if !log_path.exists() {
                break File::create(log_path)?;
            }
        };

        Ok(file)
    }

    /// Open Scoop app repository
    ///
    /// # Errors
    /// - The Scoop app could not be opened as a repository
    fn open_repo(&self) -> Option<git::Result<git::Repo>> {
        Some(git::Repo::scoop_app(self))
    }

    /// Get the path to the context's app
    ///
    /// In the case of the user context, this is the path to the scoop app
    fn context_app_path(&self) -> PathBuf {
        self.apps_path().join("scoop").join("current")
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
