use std::{
    io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Clone, Debug, Serialize, Deserialize)]
pub enum DocsError {
    #[error("Unknown locale: {locale}")]
    UnknownLocale { locale: String },

    #[error("Failed to read docs file {path:?}: {reason}")]
    FailedRead { path: PathBuf, reason: String },

    #[error("Invalid docs file format: {reason}")]
    InvalidFormat { reason: String },
}

impl DocsError {
    pub fn unknown_locale(locale: impl AsRef<str>) -> Self {
        Self::UnknownLocale {
            locale: locale.as_ref().to_string(),
        }
    }

    pub fn failed_read(path: impl AsRef<Path>, error: io::Error) -> Self {
        Self::FailedRead {
            path: path.as_ref().to_path_buf(),
            reason: error.to_string(),
        }
    }

    pub fn invalid_format(reason: impl AsRef<str>) -> Self {
        Self::InvalidFormat {
            reason: reason.as_ref().to_string(),
        }
    }
}

#[derive(Error, Clone, Debug, Serialize, Deserialize)]
pub enum Error {
    #[error(transparent)]
    Docs { error: DocsError },
}

impl From<DocsError> for Error {
    fn from(value: DocsError) -> Self {
        Self::Docs { error: value }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
