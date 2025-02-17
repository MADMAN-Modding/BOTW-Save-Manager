//! This module is used for read and writing the json data used for the overlays and the app
use std::{fs, path::Path};

use serde_json::{json, Value};

use crate::constants;

/// Reads the config json and returns the value of the requested key
/// 
/// # Arguments
/// * `key: &str` - The key to be read from the json file
/// 
/// # Returns
/// * `String` - The data at the desired key 
#[tauri::command]
pub fn read_config_json(key: &str) -> String {
    read_json(key, constants::get_config_json_path())
}

/// Reads the json at the supplied path and returns the value of the requested key
/// 
/// # Arguments
/// * 'key: &str' - The key to be read from the json file
/// * 'path': String' - The path to the json file
/// 
/// #Returns
/// * 'String' - The data at the desired key
#[tauri::command]
pub fn read_json(key: &str, path: String) -> String {
    let json_data: Value = open_json(path);

    json_data[key].to_string().replace("\"", "")
}

/// Opens the json file with the supplied path
/// 
/// # Arguments
/// * `path: String` - The path to the JSON file to read
/// 
/// # Returns
/// * `Value` - Contains the JSON data
fn open_json(path: String) -> Value {
    let json_data: Value;

    // Checks to make sure that the JSON file is there, if it isn't it makes it
    if Path::new(&path).exists() {
        json_data = {
            let file_content: String = fs::read_to_string(&path).expect("File not found");
            serde_json::from_str::<Value>(&file_content).expect("Error serializing to JSON")
        };
    } else {
        json_data = init_json(path);
    }

    // Returns the json data
    json_data
}

/// This function is called if the JSON being read doesn't exist
/// 
/// It after making the file it will try to read the file and then return that value
/// 
/// # Arguments
/// * `path: String` - The path to the JSON file to read
/// 
/// # Returns
/// * `Value` - Contains the JSON data
pub fn init_json(path: String) -> Value {
    // Creating the directories
    let _ = std::fs::create_dir_all(Path::new(&path).parent().unwrap());

    // Default json values 
    let json_data: Value = get_default_json_data();
    
    // Creating the JSON file
    fs::write(
        &path,
        serde_json::to_string_pretty(&json_data).expect(
            "Error 
    serializing to JSON",
        ),
    )
    .expect("Error writing file");

    // Trying to open the JSON again
    open_json(path)
}

/// Writes to the JSON file at the supplied path
/// 
/// # Arguments
/// * `path: String` - Path to the JSON file
/// * `json_key: String` - Key to write to
/// * `value: String` ` Value to write to the key`
/// 
/// # Examples
/// ```ignore
/// write_json("random_path/overlay.json", "overlay", "kart");
/// ```
#[tauri::command]
pub fn write_json(path: String, json_key: String, mut value: String) {
    // Cloning the data because a borrow won't work in this case
    let mut json_data = open_json(path.clone());

    // Replace all \" with nothing
    value = value.replace("\"", "");

    // Set the value in the array equal to the supplied value
    json_data[json_key] = serde_json::Value::String(value);
    
    // Write the new json data
    fs::write(
        path,
        serde_json::to_string_pretty(&json_data).expect("Error serializing to JSON"),
    )
    .expect("Error writing file");
}

/// Writes the config.json file
/// 
/// # Arguments
/// * key: String - key to update
/// * value: String - value to update the key to
#[tauri::command]
pub fn write_config(key: String, value: &str) {
    write_json(constants::get_config_json_path(), key, value.to_string().replace("\\\\", "\\"));
}

pub fn get_default_json_data() -> serde_json::Value {
    json!({
        "mlcPath": "NOT_SET",
        "backupCurrentSave" : "true"
    })
}