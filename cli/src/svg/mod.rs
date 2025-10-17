use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Svg {
  pub source_svg_path: PathBuf,
  pub svg_string: String,
}

impl Svg {
  pub fn new(
    source_svg_path: &PathBuf,
    //raw_width: u32,
    //raw_height: u32,
  ) -> Svg {
    Svg {
      //raw_height,
      //raw_width,
      source_svg_path: source_svg_path.clone(),
      svg_string: fs::read_to_string(&source_svg_path).unwrap(),
    }
  }
}
