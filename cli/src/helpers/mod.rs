use anyhow::Result;
use regex::Regex;

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
