use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

pub fn cache_is_stale(source_path: &PathBuf, cache_path: &PathBuf) -> bool {
    if !cache_path.is_file() {
        true
    } else {
        if let (Ok(source_data), Ok(cache_data)) =
            (fs::metadata(&source_path), fs::metadata(&cache_path))
        {
            if let (Ok(source_mod), Ok(cache_mod)) = (source_data.modified(), cache_data.modified())
            {
                if source_mod > cache_mod {
                    true
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            true
        }
    }
}

pub fn clean_for_url(source: &str) -> Result<String> {
    let source = source.to_lowercase();
    let re = Regex::new(r"\W").unwrap();
    let source = re.replace_all(&source, "-");
    let re = Regex::new(r"--+").unwrap();
    let source = re.replace_all(&source, "-");
    let re = Regex::new(r"^-+").unwrap();
    let source = re.replace_all(&source, "");
    let re = Regex::new(r"-+$").unwrap();
    let source = re.replace_all(&source, "");
    Ok(source.to_string())
}
