use std::{error::Error, fs::{self, create_dir_all}, path::Path};

use crate::{
    constants::{get_data_dir, get_save_dir},
    dir_lister::get_files_in_dir,
};

#[tauri::command]
pub fn new_save(name: String) -> Result<(), String> {
    let save_dir = get_save_dir();

    if Path::new(&name).exists() {
        return Err("Save Already Exists".to_string())
    }

    let new_save_dir = format!("{}/{}", get_data_dir(), name);

    let _ = create_dir_all(new_save_dir.clone());

    // Files and folders Vectors for the files to be copied
    let files = get_files_in_dir(save_dir.clone()).map_err(|err| format!("File read error: {}", err))?;

    // Copies the files
    for file in files {
        let file_path = format!("{}/{}", save_dir, file);

        let _ = fs::copy(file_path, format!("{}/{}", new_save_dir.clone(), file))
            .map_err(|err| return format!("File Copy Error: {}", err))?;
    }


    copy_directory(save_dir, new_save_dir).map_err(|e| e.to_string())?;

    Ok(())
}

/// Deletes the save
/// 
/// # Arguments
/// * save: String - name of the save to remove
/// 
/// # Returns
/// * Ok(()) - If the removal is successful
/// * Err(String) - If the load has an IO error
#[tauri::command]
pub fn remove_save(save: String) -> Result<(), String> {
    let save = format!("{}/{}", get_data_dir(), save);

    fs::remove_dir_all(save).map_err(|e| e.to_string())?;

    return Ok(());
}

/// Loads the save
/// 
/// Deletes the current save first
/// 
/// # Arguments
/// * save: String - name of the save to load
/// 
/// # Returns
/// * Ok (()) - If the load is successful
/// * Err(String) - If the load has an IO error
#[tauri::command]
pub fn load_save(save: String) -> Result<(), String> {
    fs::remove_dir_all(get_save_dir()).map_err(|e| e.to_string())?;

    copy_directory(format!("{}/{}", get_data_dir(), save), get_save_dir()).map_err(|e| e.to_string())?;

    Ok(())
}

/// Recursively copies the whole directory
/// 
/// # Arguments
/// * from: String - directory to be copied
/// * to: String - directory to copy to
/// 
/// # Returns
/// * Ok(()) - If the copy is successful
/// * Box<dyn Error> - If there is an IO error
fn copy_directory(from: String, to: String) -> Result<(), Box<dyn Error>> {
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