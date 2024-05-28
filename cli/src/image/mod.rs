pub mod mocks;

use crate::helpers::*;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
// use anyhow::Error;
//use rimage::image::io::Reader;
//use std::collections::BTreeSet;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Image {
    // TODO: Remove config
    // pub config: SiteConfig,
    pub height: Option<u32>,
    pub source_path: PathBuf,
    pub width: Option<u32>,
    pub versions: Vec<(u32, u32)>,
}

impl Image {
    pub fn extension(&self) -> Result<String> {
        Ok(self
            .source_path
            .extension()
            .expect("Could not get extension")
            .to_string_lossy()
            .to_string()
            .to_lowercase())
    }

    pub fn key(&self) -> Result<String> {
        let stem = &self
            .source_path
            .file_stem()
            .expect("could not get file name");
        clean_for_url(&stem.to_string_lossy().to_string())
    }

    pub fn set_dimensions(&mut self, widths: Vec<u32>) -> Result<()> {
        for width in widths.iter() {
            if *width < self.width.expect("width") {
                let height = self.height.expect("height") * width / self.width.expect("width");
                self.versions.push((*width, height));
            }
        }
        if let Some(biggest_request) = widths.iter().max() {
            if self.width.expect("width") < *biggest_request {
                self.versions
                    .push((self.width.expect("width"), self.height.expect("height")))
            }
        }
        Ok(())
    }

    // DEPRECATED: remove when image is working
    // pub fn load_width_and_height(&mut self) -> Result<()> {
    //     let img = Reader::open(&self.source_path)?;
    //     let data = img.decode()?;
    //     self.width = Some(data.width());
    //     self.height = Some(data.height());
    //     Ok(())
    // }

    // DEPRECATED: remove when image is working
    // pub fn output_widths(&self) -> BTreeSet<u32> {
    //     let mut widths = BTreeSet::new();
    //     self.config.theme.images.iter().for_each(|i| {
    //         i.widths.iter().for_each(|w| {
    //             if *w < self.width.unwrap() {
    //                 widths.insert(w.clone());
    //             }
    //         });
    //     });
    //     widths
    // }

    //
}
