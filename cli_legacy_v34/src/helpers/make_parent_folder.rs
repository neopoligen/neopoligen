use std::path::Path;
use std::fs;

pub fn make_parent_folder(file_path: &Path) -> Result<String, String> {
    match file_path.parent() {
        Some(parent_folder) => match fs::create_dir_all(parent_folder) {
            Ok(_) => Ok("Made the folder".to_string()),
            Err(e) => Err(e.to_string()),
        },
        None => Err("Could not make folder".to_string()),
    }
}
