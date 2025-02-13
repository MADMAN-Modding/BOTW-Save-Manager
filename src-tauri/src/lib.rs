pub mod dir_lister;
pub mod constants;
pub mod save_handler;
pub mod json_handler;
pub mod image_handler;
pub mod thread_data;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Setups for different modules
    constants::setup();
    dir_lister::setup();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            save_handler::new_save,
            save_handler::remove_save,
            save_handler::load_save,
            dir_lister::get_folders_in_dir,
            dir_lister::start_search,
            constants::get_data_dir,
            image_handler::get_image_bytes,
            json_handler::read_config_json,
            json_handler::write_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
