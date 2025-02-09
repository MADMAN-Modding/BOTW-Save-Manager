use std::{fs::{self, create_dir_all}, path::Path};

use crate::{
    constants::{get_data_dir, get_mlc_dir},
    dir_lister::get_files_in_dir,
};

#[tauri::command]
pub fn new_backup(name: String) -> Result<(), String> {
    let mlc_path = get_mlc_dir();

    if Path::new(&name).exists() {
        return Err("Backup Already Exists".to_string())
    }

    let backup_dir = format!("{}/{}", get_data_dir(), name);

    let _ = create_dir_all(backup_dir.clone());

    // Files and folders Vectors for the files to be copied
    let files = get_files_in_dir(get_mlc_dir()).map_err(|err| format!("File read error: {}", err))?;

    // Copies the files
    for file in files {
        let file_path = format!("{}/{}", mlc_path, file);

        let _ = fs::copy(file_path, format!("{}/{}", backup_dir.clone(), file))
            .map_err(|err| return format!("File Copy Error: {}", err))?;
    }


    copy_directory(mlc_path, backup_dir).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn remove_backup(path: String) -> Result<(), String> {
    let path = format!("{}/{}", get_data_dir(), path);

    fs::remove_dir_all(path).map_err(|e| e.to_string())?;

    return Ok(());
}

fn copy_directory(from: String, to: String) -> Result<(), Box<dyn std::error::Error>> {
    // Create the target directory
    create_dir_all(&to)?;

    for entry in fs::read_dir(&from)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();

        let source_path = format!("{}/{}", from, file_name);
        let dest_path = format!("{}/{}", to, file_name);

        if entry.file_type()?.is_dir() {
            copy_directory(source_path, dest_path)?;
        } else {
            fs::copy(&source_path, &dest_path)?;
        }
    }

    Ok(())
}