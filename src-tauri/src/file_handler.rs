use std::{fs::{self, create_dir_all}, path::Path};

use crate::{constants::{get_data_dir, get_mlc_dir}, dir_lister::{get_files_in_dir, get_folders_in_dir}};

pub fn new_backup(name: String) -> Result<(), String> {
    let mlc_path = get_mlc_dir();

    if Path::new(&name).exists() {
        return Err("Backup Already Exists".to_string())
    }

    let backup_dir = format!("{}/{}", get_data_dir(), name);

    let _ = create_dir_all(backup_dir);

    let files = get_files_in_dir().map_err(|err| err);
    let folders = get_folders_in_dir().map_err(|err| err);

    // Copies the files
    for file in files.unwrap() {
        let _ = fs::copy(format!("{}/{}", mlc_path, file), format!("{}/{}", get_data_dir(), name)).map_err(|err| format!("Copy Error: {}", err));
    }

    // Copies the folders
    for folder in folders.unwrap() {
        let _ = fs::copy(format!("{}/{}", mlc_path, folder), format!("{}/{}", get_data_dir(), name)).map_err(|err| format!("Copy Error: {}", err));
    }

    Ok(())
}