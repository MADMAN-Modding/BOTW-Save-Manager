//! Handles the `Esports-Logo.png` file
use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::constants::get_data_dir;

/// Returns the bytes of the image in the code dir as bytes
/// 
/// These bytes are used to display the image on the frontend
/// 
/// # Returns
/// * `Ok(Vec<u8>>` - If the read is successful it will send the data to the frontend
/// * 'Err(String)` - If the read fails, it will send the error to the frontend and the default image will be displayed
#[tauri::command]
pub fn get_image_bytes(path: String) -> Result<Vec<u8>, String> {
    let image_path = format!("{}/{}/0/caption.jpg", get_data_dir(), path);

    let image: Result<File, std::io::Error> =
        File::open(image_path);

    if image.is_err() {
        return Err(image.unwrap_err().to_string());
    }
    
    let mut reader: BufReader<File> = BufReader::new(image.unwrap());

    let mut buffer: Vec<u8> = Vec::new();

    let read_result = reader.read_to_end(&mut buffer);

    match read_result {
        Ok(_) => Ok(buffer),
        Err(error) => Err(error.to_string()),
    }

}