use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(thiserror::Error, Serialize, Deserialize, Type, Clone, Debug)]
#[serde(tag = "path")]
pub enum CommandError {}
