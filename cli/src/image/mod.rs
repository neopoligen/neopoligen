use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Image {
  // pub raw_height: u32,
  // pub raw_width: u32,
  pub source_image_path: PathBuf,
  pub source_image_timestamp: u64,
  // pub extension: String,
  // pub dir: PathBuf,
  // pub sizes: Vec<ImageSize>,
}

impl Image {
  pub fn new(
    source_image_path: &PathBuf,
    source_image_timestamp: u64,
    //raw_width: u32,
    //raw_height: u32,
  ) -> Image {
    Image {
      //raw_height,
      //raw_width,
      source_image_path: source_image_path.clone(),
      source_image_timestamp,
    }
  }
}

// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// pub struct ImageSize {
//     pub width: u32,
//     pub height: u32,
// }
