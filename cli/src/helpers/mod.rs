use std::fs;
use std::path::PathBuf;

pub fn write_file_with_mkdir(path: &PathBuf, content: &str) -> Result<(), String> {
    match path.parent() {
        Some(parent_dir) => match fs::create_dir_all(parent_dir) {
            Ok(_) => match fs::write(path, content) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        },
        None => Err("Could not make directory".to_string()),
    }
}
