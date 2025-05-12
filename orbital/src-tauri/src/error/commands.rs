use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(thiserror::Error, Serialize, Deserialize, Type, Clone, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AssetVersionError {}

#[derive(thiserror::Error, Serialize, Deserialize, Type, Clone, Debug)]
#[serde(tag = "path")]
pub enum CommandError {
    #[error(transparent)]
    #[serde(rename = "api.asset_versions")]
    AssetVersion {
        #[serde(flatten)]
        error: AssetVersionError
    }
}
