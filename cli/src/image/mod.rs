pub mod mocks;

// use anyhow::Error;
use crate::{helpers::*, site_config::SiteConfig};
use anyhow::Result;
use rimage::image::io::Reader;
use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Image {
    pub config: SiteConfig,
    pub height: Option<u32>,
    pub source_path: PathBuf,
    pub width: Option<u32>,
}

impl Image {
    pub fn key(&self) -> Result<String> {
        let stem = &self
            .source_path
            .file_stem()
            .expect("could not get file name");
        clean_for_url(&stem.to_string_lossy().to_string())
    }

    pub fn load_width_and_height(&mut self) -> Result<()> {
        let img = Reader::open(&self.source_path)?;
        let data = img.decode()?;
        self.width = Some(data.width());
        self.height = Some(data.height());
        Ok(())
    }

    pub fn output_widths(&self) -> BTreeSet<u32> {
        let mut widths = BTreeSet::new();
        self.config.theme.images.iter().for_each(|i| {
            i.widths.iter().for_each(|w| {
                if *w < self.width.unwrap() {
                    widths.insert(w.clone());
                }
            });
        });
        widths
    }

    //
}
