//! Helpers to maintain parity between git2 and gitoxide during the transition

use chrono::{DateTime, FixedOffset};

#[derive(Clone, PartialEq, Eq)]
/// A wrapper around a git signature that supports git2 and gitoxide
pub enum Signature {
    /// A git2 signature
    Git2(git2::Signature<'static>),
    /// A gitoxide signature
    Gitoxide(gix::actor::Signature),
}

impl<'a> From<git2::Signature<'a>> for Signature {
    fn from(signature: git2::Signature<'a>) -> Self {
        Self::Git2(signature.to_owned())
    }
}

impl From<gix::actor::SignatureRef<'_>> for Signature {
    fn from(signature: gix::actor::SignatureRef<'_>) -> Self {
        Self::Gitoxide(signature.to_owned())
    }
}

impl From<gix::actor::Signature> for Signature {
    fn from(signature: gix::actor::Signature) -> Self {
        Self::Gitoxide(signature)
    }
}

/// A wrapper around a git signature that supports git2 and gitoxide
pub enum Time {
    /// A git2 time
    Git2(git2::Time),
    /// A gitoxide time
    Gitoxide(gix::date::Time),
}

impl From<git2::Time> for Time {
    fn from(time: git2::Time) -> Self {
        Self::Git2(time)
    }
}

impl From<gix::date::Time> for Time {
    fn from(time: gix::date::Time) -> Self {
        Self::Gitoxide(time)
    }
}

impl Time {
    #[must_use]
    /// Get the time as a datetime
    pub fn to_datetime(&self) -> Option<DateTime<FixedOffset>> {
        match self {
            Time::Git2(time) => {
                let utc_time = DateTime::from_timestamp(time.seconds(), 0)?;
                let offset = FixedOffset::east_opt(time.offset_minutes() * 60)?;

                Some(utc_time.with_timezone(&offset))
            }
            Time::Gitoxide(time) => {
                let utc_time = DateTime::from_timestamp(time.seconds, 0)?;
                let offset = FixedOffset::east_opt(time.offset)?;

                Some(utc_time.with_timezone(&offset))
            }
        }
    }
}

/// A commit, either git2 or gitoxide
pub enum Commit<'a> {
    /// A git2 commit
    Git2(git2::Commit<'a>),
    /// A gitoxide commit
    Gitoxide(gix::Commit<'a>),
}

impl Commit<'_> {
    /// Get the time of the commit
    pub fn time(&self) -> Option<Time> {
        match self {
            Commit::Git2(commit) => Some(commit.time().into()),
            Commit::Gitoxide(commit) => commit.time().map(Into::into).ok(),
        }
    }

    /// Get the author of the commit
    pub fn author(&self) -> Option<Signature> {
        match self {
            Commit::Git2(commit) => Some(commit.author().into()),
            Commit::Gitoxide(commit) => commit.author().ok().map(Into::into),
        }
    }
}
