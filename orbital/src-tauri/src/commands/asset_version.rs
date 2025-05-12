use orbital_common::Result;
use tauri::{AppHandle, Runtime, Wry};

#[taurpc::procedures(path = "asset_versions")]
trait AssetVersionsApi {
    async fn list_available(app_handle: AppHandle<impl Runtime>) -> Result<Vec<String>>;
}
