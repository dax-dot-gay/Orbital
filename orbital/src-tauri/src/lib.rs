mod commands;

use commands::routes;
pub mod error;
pub mod utils;

pub use error::{Error, Result};
pub(crate) use error::*;

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_persistence::init())
        .plugin(tauri_plugin_zustand::init())
        .invoke_handler(routes())
        .setup(|app| {
            utils::AppState::attach(app.handle());

            Ok(())
        });

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
