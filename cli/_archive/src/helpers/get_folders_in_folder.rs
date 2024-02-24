use std::fs;
use std::path::PathBuf;

pub fn get_folders_in_folder(dir: &PathBuf) -> Vec<PathBuf> {
    // dbg!(&dir);
    fs::read_dir(dir)
        .unwrap()
        .filter(|p| p.as_ref().unwrap().path().is_dir())
        .filter_map(|p| match p.as_ref().unwrap().path().strip_prefix(".") {
            Ok(_) => None,
            Err(_) => Some(p.as_ref().unwrap().path()),
        })
        .collect()
}
