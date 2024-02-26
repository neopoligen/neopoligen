use std::fs;
use std::path::PathBuf;

pub fn get_folders_in_folder(dir: &PathBuf) -> Vec<PathBuf> {
    // dbg!(&dir);
    match fs::read_dir(dir) {
        Ok(d) => d
            .filter(|p| p.as_ref().unwrap().path().is_dir())
            .filter_map(|p| match p.as_ref().unwrap().path().strip_prefix(".") {
                Ok(_) => None,
                Err(_) => Some(p.as_ref().unwrap().path()),
            })
            .collect(),
        Err(e) => {
            println!("{}", e);
            vec![]
        }
    }
}
