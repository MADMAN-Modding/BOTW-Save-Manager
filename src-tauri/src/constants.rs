use std::fs::create_dir_all;

use directories::ProjectDirs;
use once_cell::sync::OnceCell;

use crate::json_handler::read_config_json;

static PROJ_DIRS: OnceCell<ProjectDirs> = OnceCell::new();

pub fn setup() {
    PROJ_DIRS
        .set(
            ProjectDirs::from(
                "com",
                "MADMAN-Modding",
                "BOTW Save Manager",
            )
            .expect("Failed to create PROJ_DIRS"),
        )
        .unwrap();

    gen_dirs(PROJ_DIRS.get().expect("PROJ_DIRS has nto been initialized"));
}

/// Directory for program config
pub fn get_config_dir() -> String {
    let proj_dir = PROJ_DIRS.get().expect("PROJ_DIRS has not been initialized");

    ProjectDirs::config_dir(proj_dir).to_str().unwrap().to_string()
}

/// Directory for program data
/// 
/// eg: Saves
#[tauri::command]
pub fn get_data_dir() -> String {
    let proj_dir = PROJ_DIRS.get().expect("PROJ_DIRS has not been initialized");

    ProjectDirs::data_dir(proj_dir).to_str().unwrap().to_string()
}

pub fn get_config_json_path() -> String {
    format!("{}/config.json", get_config_dir())
}

/// Makes the directories for the program to use
fn gen_dirs(proj_dir: &ProjectDirs) {
    let _ = create_dir_all(proj_dir.config_dir());
    
    let _ = create_dir_all(proj_dir.data_dir());
}

pub fn get_mlc_dir() -> String {
    read_config_json("mlcPath")
}