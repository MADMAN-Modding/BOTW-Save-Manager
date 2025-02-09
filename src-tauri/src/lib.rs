pub mod dir_lister;
pub mod constants;
pub mod backup_handler;
pub mod json_handler;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Setups for different modules
    constants::setup();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            backup_handler::new_backup
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
