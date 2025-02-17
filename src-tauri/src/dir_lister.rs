use std::{env, error::Error, fs::{self, ReadDir}, sync::{Arc, Mutex}, thread};

use once_cell::sync::OnceCell;

use crate::thread_data::ThreadData;

static THREAD_DATA: OnceCell<Arc<Mutex<ThreadData>>> = OnceCell::new();

pub fn setup() {
    let _ = THREAD_DATA.set(Arc::new(Mutex::new(ThreadData::setup())));
}

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

/// Starts the search for the MLC01 folder
/// 
/// **Returns**
/// * String - Path to the MLC01 folder
/// * String - Error message
#[tauri::command]
pub async fn start_search() -> Result<String, String> {
    let thread_data = THREAD_DATA.get().unwrap();

    // Sets the path to search based on the OS
    let path = match env::consts::OS.to_string().as_str() {
        "windows" => "C:/",
        "linux" => "/",
        "macos" => "/",
        _ => "/"
    };
    
    // Makes the thread
    let thread =
    thread::spawn(|| ThreadData::find_mlc01(thread_data.clone(), path.to_string()));
    
    // Result from the thread
    let result = thread.join().unwrap();

    // Allows future scanning to take place
    thread_data.lock().unwrap().set_stop(false);

    // Returns the result
    match result {
        Ok(value) => Ok(value),
        Err(error) => Err(error)
    }

}

impl ThreadData {
    /// Finds the MLC01 folder
    /// 
    /// **Arguments**
    /// * thread_data: Arc<Mutex<Self>> - ThreadData object
    /// 
    /// **Returns**
    /// * String - Path to the MLC01 folder
    /// * String - Error message
    fn find_mlc01(thread_data: Arc<Mutex<Self>>, path: String) -> Result<String, String> {
        
        let mut mlc01: Result<String, String> = Ok("NOT_SET".to_string());

        for drive in fs::read_dir(&path).map_err(|e| e.to_string())? {
            let drive = drive.map_err(|e| e.to_string())?;

            if thread_data.lock().unwrap().get_stop() == true {return mlc01}

            if drive.file_name().eq("mlc01") {
                mlc01 = Ok(drive.path().to_str().unwrap().to_string());
                thread_data.lock().unwrap().set_stop(true);
                return mlc01;
            } else {
                if drive.file_type().unwrap().is_dir() {
                    mlc01 = Self::find_mlc01(thread_data.clone(), drive.path().to_str().unwrap().to_string());
                }
            }
        }
        mlc01
    }
}