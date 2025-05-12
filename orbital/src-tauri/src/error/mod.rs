use serde::{Deserialize, Serialize};
use specta::Type;

mod commands;

#[derive(thiserror::Error, Serialize, Deserialize, Type, Clone, Debug)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum Error {
    #[error(transparent)]
    CommonInternals {
        #[serde(flatten)]
        error: orbital_common::Error,
    },

    #[error(transparent)]
    Command {
        #[serde(flatten)]
        error: commands::CommandError,
    },
}
