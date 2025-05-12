use std::fs;

use tauri::{AppHandle, Manager, Runtime};

#[taurpc::procedures(path = "asset_versions")]
pub trait AssetVersionsApi {
    async fn list_available(app_handle: AppHandle<impl Runtime>) -> crate::Result<Vec<String>>;
}

#[derive(Clone)]
pub struct AssetVersionsImpl;

#[taurpc::resolvers]
impl AssetVersionsApi for AssetVersionsImpl {
    async fn list_available(self, app_handle: AppHandle<impl Runtime>) -> crate::Result<Vec<String>> {
        let asset_dir = app_handle.path().resource_dir().or_else(|e| Err(crate::Error::from(e)))?.join("resources/assets");
        if !asset_dir.exists() {
            return Err(crate::OperationError::invalid_path(asset_dir.as_path(), crate::InvalidPathType::NotExists));
        }
        if !asset_dir.is_dir() {
            return Err(crate::OperationError::invalid_path(asset_dir.as_path(), crate::InvalidPathType::ExpectedFolder));
        }

        let read_dir = fs::read_dir(asset_dir).or_else(|e| Err(crate::Error::from(e)))?;

        Ok(read_dir.filter_map(|entry| {
            if let Ok(e) = entry {
                if e.path().is_dir() {
                    Some(e.file_name().into_string().unwrap())
                } else {
                    None
                }
            } else {
                None
            }
        }).collect())
    }
}
