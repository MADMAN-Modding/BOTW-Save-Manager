use std::{error::Error, fs::{self, ReadDir}};

/// Gets the folders inside the directory
/// 
/// **Returns**
/// * Vec<String> - Vec of all the folders
/// * Box<dyn Error> - Error reading the directory
#[tauri::command]
pub fn get_folders_in_dir(path: String) -> Result<Vec<String>, String> {
    get_items(false, path).map_err(|e| e.to_string())
}

/// Gets the files inside the directory
/// 
/// **Returns**
/// * Vec<String> - Vec of all the files
/// * Box<dyn Error> - Error reading the directory
pub fn get_files_in_dir(path: String) -> Result<Vec<String>, Box<dyn Error>> {
    get_items(true, path)
}

/// Gets the files or folders inside the directory
/// 
/// **Arguments**
/// * files: bool - true for returning files, false for folders
/// 
/// **Returns**
/// * Vec<String> - Vec of all the items
/// * Box<dyn Error> - Error reading the directory
fn get_items(files: bool, path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut items: Vec<String> = Vec::new();

    match get_dir(path) {
        Ok(dir) => {
            for item in dir {
                if item.as_ref().unwrap().file_type().unwrap().is_file() == files {
                    items.push(item.unwrap().file_name().into_string().unwrap());
                }
            } 
        },
        Err(err) => return Err(err)
    }

    Ok(items)
}

/// ReadDir object for the path of the saves
/// 
/// **Returns**
/// * ReadDir - If the dir exists
/// * Box<dyn Error> - If the dir doesn't exist
fn get_dir(path: String) -> Result<ReadDir, Box<dyn Error>> {
    let dir = fs::read_dir(path)?;

    Ok(dir)
}