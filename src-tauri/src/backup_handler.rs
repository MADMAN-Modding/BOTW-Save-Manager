use std::{fs::{self, create_dir_all}, path::Path};

use crate::{constants::{get_data_dir, get_mlc_dir}, dir_lister::{get_files_in_dir, get_folders_in_dir}};

#[tauri::command]
pub fn new_backup(name: String) -> Result<(), String> {
    let mlc_path = get_mlc_dir();

    // if Path::new(&name).exists() {
    //     return Err("Backup Already Exists".to_string())
    // }

    let backup_dir = format!("{}/{}", get_data_dir(), name);

    let _ = create_dir_all(backup_dir.clone());

    // Files and folders Vectors for the files to be copied
    let files = get_files_in_dir().map_err(|err| err);
    let folders = get_folders_in_dir().map_err(|err| err);

    // Copies the files
    for file in files.unwrap() {
        let file_path = format!("{}/{}", mlc_path, file);
        println!("{}", file_path);
        println!("{}", backup_dir.clone());

        let copy = fs::copy(file_path, backup_dir.clone()).map_err(|err| return format!("Copy Error: {}", err));

        if copy.is_err() {
            println!("{}", copy.unwrap_err())
        }
    }

    // Copies the folders
    for folder in folders.unwrap() {
        let copy = fs::copy(format!("{}/{}", mlc_path, folder), backup_dir.clone()).map_err(|err| format!("Copy Error: {}", err));
        
        if copy.is_err() {
            println!("{}", copy.unwrap_err())
        }
    }

    Ok(())
}