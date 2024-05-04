use std::collections::BTreeMap;
use std::path::PathBuf;

pub struct Builder {
    input_files: BTreeMap<PathBuf, String>,
}
