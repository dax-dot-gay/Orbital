use std::{fmt::Display, io, path::Path, sync::Arc};

use serde::{Deserialize, Serialize};
use specta::Type;
mod commands;

pub use commands::*;

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InvalidPathType {
    Format,
    Exists,
    NotExists,
    ExpectedFile,
    ExpectedFolder
}

impl Display for InvalidPathType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}

#[derive(thiserror::Error, Serialize, Deserialize, Type, Clone, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OperationError {
    #[error("Invalid path {path}: {invalid}")]
    InvalidPathError {
        path: String,
        invalid: InvalidPathType
    },

    #[error("FS failed: {reason}")]
    Filesystem {
        reason: String,

        #[serde(skip)]
        error: Option<Arc<io::Error>>
    },

    #[error("Serialization failed: {reason}")]
    Serialization {
        reason: String
    },

    #[error("Deserialization failed: {reason}")]
    Deserialization {
        reason: String
    }
}

impl OperationError {
    pub fn invalid_path(path: impl AsRef<Path>, reason: InvalidPathType) -> Error {
        Self::InvalidPathError { path: path.as_ref().to_str().unwrap_or("BAD_PATH").to_string(), invalid: reason }.into()
    }
}

#[derive(thiserror::Error, Serialize, Deserialize, Type, Clone, Debug)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum Error {
    #[error("Encountered an error in backend utilities: {error}")]
    CommonInternals {
        #[serde(flatten)]
        #[from]
        error: orbital_common::CommonError,
    },

    #[error("Encountered a command error: {error}")]
    Command {
        #[serde(flatten)]
        #[from]
        error: commands::CommandError,
    },

    #[error("Encountered a common operation error: {error}")]
    Operation {
        #[serde(flatten)]
        #[from]
        error: OperationError
    },

    #[error("Internal error in a Tauri API: {reason}")]
    Tauri {
        reason: String,

        #[serde(skip)]
        error: Option<Arc<tauri::Error>>
    },

    #[error("Encountered an error in the Persistence API: {error:?}")]
    Persistence {
        #[serde(flatten)]
        #[from]
        error: tauri_plugin_persistence::Error
    }
}

impl From<tauri::Error> for Error {
    fn from(value: tauri::Error) -> Self {
        Self::Tauri { reason: format!("{value:?}"), error: Some(Arc::new(value)) }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Operation { error: OperationError::Filesystem { reason: value.to_string(), error: Some(Arc::new(value)) } }
    }
}

impl Error {
    pub fn serialization(error: impl serde::ser::Error) -> Self {
        Self::Operation { error: OperationError::Serialization { reason: error.to_string() } }
    }

    pub fn deserialization(error: impl serde::de::Error) -> Self {
        Self::Operation { error: OperationError::Deserialization { reason: error.to_string() } }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
