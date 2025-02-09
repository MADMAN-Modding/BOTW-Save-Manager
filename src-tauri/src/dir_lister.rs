use std::{error::Error, fs::{self, ReadDir}};

/// Gets the folders inside the directory
/// 
/// **Returns**
/// * Vec<String> - Vec of all the folders
/// * Box<dyn Error> - Error reading the directory
pub fn get_folders_in_dir() -> Result<Vec<String>, Box<dyn Error>> {
    get_items(false)
}

/// Gets the files inside the directory
/// 
/// **Returns**
/// * Vec<String> - Vec of all the files
/// * Box<dyn Error> - Error reading the directory
pub fn get_files_in_dir() -> Result<Vec<String>, Box<dyn Error>> {
    get_items(true)
}

/// Gets the files or folders inside the directory
/// 
/// **Arguments**
/// * files: bool - true for returning files, false for folders
/// 
/// **Returns**
/// * Vec<String> - Vec of all the items
/// * Box<dyn Error> - Error reading the directory
fn get_items(files: bool) -> Result<Vec<String>, Box<dyn Error>> {
    let mut items: Vec<String> = Vec::new();

    match get_dir() {
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
fn get_dir() -> Result<ReadDir, Box<dyn Error>> {
    let dir = fs::read_dir(r"C:\Users\mad\Documents\Emulators\Wii U\Cemu\mlc01\usr\save\00050000\101c9400\user\80000001")?;

    Ok(dir)
}