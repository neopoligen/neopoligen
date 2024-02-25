pub mod builders;

use std::path::PathBuf;

pub struct Page {
    pub ast: String,
    pub source: String,
    pub source_path: PathBuf,
}
