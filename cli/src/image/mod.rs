use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Image {
    pub source_image_path: PathBuf,
    pub source_image_timestamp: u64,
    // pub extension: String,
    // pub dir: PathBuf,
    // pub raw_height: u32,
    // pub raw_width: u32,
    // pub sizes: Vec<ImageSize>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}
