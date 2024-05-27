pub mod mocks;

// use anyhow::Error;
use anyhow::Result;
// use rimage::image::io::Reader;
use crate::helpers::*;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Image {
    pub source_root: PathBuf,
    pub source_path: PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl Image {
    pub fn key(&self) -> Result<String> {
        let stem = &self
            .source_path
            .file_stem()
            .expect("could not get file name");
        clean_for_url(&stem.to_string_lossy().to_string())
    }

    // pub fn load_with_width_and_height(source_path: &PathBuf, source_root: &PathBuf) -> Result<Image> {
    //     let img = Reader::open(&source_path)?;
    //     let data = img.decode()?;
    //     Ok(Image {
    //         source_root: source_root.clone(),
    //         source_path: source_path.clone(),
    //         width: Some(data.width()),
    //         height: Some(data.height()),
    //     })
    // }

    //
}
