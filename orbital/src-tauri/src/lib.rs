use orbital_common::types::satisfactory::OrbitalData;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)]
    let mut specta_builder = tauri_specta::Builder::<tauri::Wry>::new()
        .typ::<OrbitalData>();

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    specta_builder
        .export(specta_typescript::Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_persistence::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(move |app| {
            specta_builder.mount_events(app);
            Ok(())
        });

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
