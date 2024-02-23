use std::fs;
use std::path::PathBuf;

pub fn get_files_with_extension_in_a_single_directory(
    dir: &PathBuf,
    extension: &str,
) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .into_iter()
        .filter(|p| {
            if p.as_ref().unwrap().path().is_file() {
                true
            } else {
                false
            }
        })
        .filter(|p| match p.as_ref().unwrap().path().extension() {
            Some(ext) => ext == extension,
            None => false,
        })
        .filter_map(|p| match p.as_ref().unwrap().path().strip_prefix(".") {
            Ok(_) => None,
            Err(_) => Some(p.as_ref().unwrap().path()),
        })
        .collect()
}
