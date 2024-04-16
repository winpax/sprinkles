//! Scoop bucket helpers

use std::{
    borrow::Cow,
    collections::HashSet,
    path::{Path, PathBuf},
};

use rayon::prelude::*;
use regex::Regex;

use crate::{
    git::{self, Repo},
    output::sectioned::{Children, Section, Text},
    packages::{self, CreateManifest, InstallManifest, Manifest, SearchMode},
    Scoop,
};

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
/// Bucket errors
pub enum Error {
    #[error("Interacting with repo: {0}")]
    RepoError(#[from] git::Error),
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("The bucket \"{0}\" does not exist")]
    InvalidBucket(PathBuf),
    #[error("Missing or invalid git output")]
    MissingGitOutput,
    #[error("Could not find executable in path: {0}")]
    MissingInPath(#[from] which::Error),
    #[error("Invalid time. (time went backwards or way way way too far forwards (hello future! whats it like?))")]
    InvalidTime,
    #[error("Invalid timezone provided. (where are you?)")]
    InvalidTimeZone,
}

/// Bucket result type
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
/// A bucket
pub struct Bucket {
    bucket_path: PathBuf,
}

impl Bucket {
    /// Open a bucket from its name
    ///
    /// # Errors
    /// - Bucket does not exist
    pub fn from_name(name: impl AsRef<Path>) -> Result<Self> {
        Self::from_path(Scoop::buckets_path().join(name))
    }

    /// Open given path as a bucket
    ///
    /// # Errors
    /// - Bucket does not exist
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let bucket_path = path.as_ref().to_path_buf();

        if bucket_path.exists() {
            Ok(Self { bucket_path })
        } else {
            Err(Error::InvalidBucket(path.as_ref().to_path_buf()))
        }
    }

    /// Open a single bucket, or return all available buckets
    ///
    /// # Errors
    /// - Any listed or provided bucket is invalid
    /// - Unable to read the bucket directory
    pub fn one_or_all(name: Option<impl AsRef<Path>>) -> Result<Vec<Self>> {
        if let Some(name) = name {
            Ok(vec![Bucket::from_name(name)?])
        } else {
            Bucket::list_all()
        }
    }

    /// Open the repository from the bucket path
    ///
    /// # Errors
    /// - The bucket could not be opened as a repository
    #[inline]
    pub fn open_repo(&self) -> Result<Repo> {
        Ok(Repo::from_bucket(self)?)
    }

    /// Gets the bucket's name (the final component of the path)
    ///
    /// # Panics
    /// If the `file_name` function returns `None`, or a non-utf8 string.
    #[must_use]
    pub fn name(&self) -> Cow<'_, str> {
        self.path()
            .file_name()
            .map(|f| f.to_string_lossy())
            .expect("File to have file name")
    }

    #[must_use]
    /// Gets the bucket's path
    pub fn path(&self) -> &Path {
        &self.bucket_path
    }

    /// Gets all buckets
    ///
    /// # Errors
    /// - Was unable to read the bucket directory
    /// - Any listed bucket is invalid
    pub fn list_all() -> Result<Vec<Bucket>> {
        let bucket_dir = std::fs::read_dir(Scoop::buckets_path())?;

        bucket_dir
            .filter(|entry| entry.as_ref().is_ok_and(|entry| entry.path().is_dir()))
            .map(|entry| Self::from_path(entry?.path()))
            .collect()
    }

    /// List all packages contained within this bucket
    ///
    /// # Errors
    /// - The bucket is invalid
    /// - Any package has an invalid path or invalid contents
    /// - See more at [`packages::PackageError`]
    pub fn list_packages(&self) -> packages::Result<Vec<Manifest>> {
        let dir = std::fs::read_dir(self.path().join("bucket"))?;

        dir.map(|manifest| Manifest::from_path(manifest?.path()))
            .collect()
    }

    /// List all packages contained within this bucket, ignoring errors
    ///
    /// # Errors
    /// - The bucket is invalid
    /// - See more at [`packages::PackageError`]
    pub fn list_packages_unchecked(&self) -> packages::Result<Vec<Manifest>> {
        let dir = std::fs::read_dir(self.path().join("bucket"))?;

        Ok(dir
            .map(|manifest| Manifest::from_path(manifest?.path()))
            .filter_map(|result| match result {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .collect())
    }

    /// List all packages contained within this bucket, returning their names
    ///
    /// # Errors
    /// - The bucket is invalid
    /// - See more at [`packages::PackageError`]
    pub fn list_package_names(&self) -> packages::Result<Vec<String>> {
        let dir = std::fs::read_dir(self.path().join("bucket"))?;

        Ok(dir
            .map(|entry| {
                entry.map(|file| {
                    file.path()
                        .with_extension("")
                        .file_name()
                        .map(|file_name| file_name.to_string_lossy().to_string())
                })
            })
            .filter_map(|file_name| match file_name {
                Ok(Some(file_name)) => Some(file_name),
                _ => None,
            })
            .collect())
    }

    /// Get the path to the manifest for the given package name
    pub fn get_manifest_path(&self, name: impl AsRef<str>) -> PathBuf {
        let buckets_path = self.path();
        let manifests_path = buckets_path.join("bucket");

        let file_name = format!("{}.json", name.as_ref());

        manifests_path.join(file_name)
    }

    /// Gets the manifest that represents the given package name
    ///
    /// # Errors
    /// - Could not load the manifest from the path
    pub fn get_manifest(&self, name: impl AsRef<str>) -> packages::Result<Manifest> {
        let manifest_path = self.get_manifest_path(name);

        Manifest::from_path(manifest_path).map(|manifest| manifest.with_bucket(self))
    }

    /// List all matches for the given pattern
    ///
    /// # Errors
    /// - Could not load the manifest from the path
    pub fn matches(
        &self,
        installed_only: bool,
        search_regex: &Regex,
        search_mode: SearchMode,
    ) -> packages::Result<Option<Section<Section<Text<String>>>>> {
        // Ignore loose files in the buckets dir
        if !self.path().is_dir() {
            return Ok(None);
        }

        let bucket_contents = self.list_package_names()?;

        let matches = bucket_contents
            .par_iter()
            .filter_map(|manifest_name| {
                // Ignore non-matching manifests
                if search_mode.eager_name_matches(manifest_name, search_regex) {
                    match self.get_manifest(manifest_name) {
                        Ok(manifest) => manifest.parse_output(
                            self.name(),
                            installed_only,
                            search_regex,
                            search_mode,
                        ),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if matches.is_empty() {
            Ok(None)
        } else {
            Ok(Some(
                Section::new(Children::from(matches))
                    // TODO: Remove quotes and bold bucket name
                    .with_title(format!("'{}' bucket:", self.name())),
            ))
        }
    }

    /// List all used buckets
    ///
    /// # Errors
    /// Invalid install manifest
    /// Reading directories fails
    pub fn used() -> packages::Result<HashSet<String>> {
        Ok(InstallManifest::list_all()?
            .par_iter()
            .filter_map(|entry| entry.bucket.clone())
            .collect())
    }

    // TODO: Check if calling this for every single bucket is slow
    /// Check if the current bucket is used
    ///
    /// # Errors
    /// Invalid install manifest
    /// Reading directories fails
    pub fn is_used(&self) -> packages::Result<bool> {
        Ok(Self::used()?.contains(&self.name().to_string()))
    }

    /// Checks if the given bucket is outdated
    ///
    /// # Errors
    /// - The bucket could not be opened as a directory
    /// - No remote named "origin"
    /// - No active branch
    /// - No reference "`FETCH_HEAD`"
    pub fn outdated(&self) -> Result<bool> {
        Ok(self.open_repo()?.outdated()?)
    }

    /// Get the number of manifests in the bucket
    ///
    /// # Errors
    /// - Could not read the bucket directory
    pub fn manifests(&self) -> Result<usize> {
        Ok(std::fs::read_dir(self.path().join("bucket"))?.count())
    }

    /// Get the bucket's source url
    ///
    /// # Errors
    /// - The bucket could not be opened as a repository
    /// - The bucket's origin remote could not be found
    /// - The remote's url is not utf8
    /// - The remote's url is not set
    pub fn source(&self) -> Result<String> {
        Ok(self
            .open_repo()?
            .origin()
            .ok_or(git::Error::MissingRemote("origin".to_string()))?
            .url()
            .map(std::string::ToString::to_string)
            .ok_or(git::Error::NonUtf8)?)
    }
}