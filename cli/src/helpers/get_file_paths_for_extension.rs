use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_file_paths_for_extension(source_dir: &PathBuf, extension: &str) -> Vec<PathBuf> {
    let walker = WalkDir::new(source_dir).into_iter();
    walker
        .filter_map(|path_result| match path_result {
            Ok(path) => match path.path().extension() {
                Some(ext) => {
                    if ext == extension {
                        Some(path.path().to_path_buf())
                    } else {
                        None
                    }
                }
                None => None,
            },
            Err(_) => None,
        })
        .collect()
}
