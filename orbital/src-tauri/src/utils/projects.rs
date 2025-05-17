use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub asset_version: String,
}

impl Project {
    pub fn new(name: impl AsRef<str>, asset_version: impl AsRef<str>) -> Self {
        let name = name.as_ref().to_string();
        let asset_version = asset_version.as_ref().to_string();
        Self {
            id: name.to_case(Case::Snake),
            name,
            asset_version,
        }
    }
}
