// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub mod dir_lister;
pub mod constants;
pub mod file_handler;
pub mod json_handler;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Setups for different modules
    constants::setup();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
