use tauri::{Runtime, Wry};

#[tauri::command(rename = "test_command")]
#[specta::specta]
pub async fn list_asset_versions(
    app: tauri::AppHandle<Wry>,
    window: tauri::Window<Wry>,
) -> Result<(), String> {
    Ok(())
}
