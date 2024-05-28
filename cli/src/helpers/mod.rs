use anyhow::Result;
use regex::Regex;
use std::path::PathBuf;

pub fn cache_is_stale(source_path: &PathBuf, cache_path: &PathBuf) -> bool {
    if !cache_path.is_file() {
        true
    } else {
        true
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
