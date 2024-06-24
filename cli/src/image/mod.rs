use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Image {
    pub extension: String,
    pub dir: PathBuf,
    pub raw_height: u32,
    pub raw_width: u32,
    // pub sizes: Vec<ImageSize>,
}

// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// pub struct ImageSize {
//     width: u32,
//     height: u32,
// }
