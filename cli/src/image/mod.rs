pub mod mocks;

use crate::helpers::*;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use xmp_toolkit::{xmp_ns, OpenFileOptions, XmpFile};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Image {
    pub alt_text: Option<String>,
    pub alt_text_extended: Option<String>,
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

    pub fn get_alt_text(&mut self) -> Result<()> {
        let mut f = XmpFile::new()?;
        f.open_file(
            self.source_path.clone(),
            OpenFileOptions::default().only_xmp().use_smart_handler(),
        )
        .or_else(|_err| {
            f.open_file(
                self.source_path.clone(),
                OpenFileOptions::default().use_packet_scanning(),
            )
        })?;
        let xmp = f.xmp().ok_or(std::fmt::Error)?;
        if let Some((value, _actual_lang)) = xmp.localized_text(
            xmp_ns::IPTC_CORE,
            "AltTextAccessibility",
            Some("en"),
            "en-US",
        ) {
            self.alt_text = Some(value.value)
        }
        if let Some((value, _actual_lang)) = xmp.localized_text(
            xmp_ns::IPTC_CORE,
            "ExtDescrAccessibility",
            Some("en"),
            "en-US",
        ) {
            self.alt_text_extended = Some(value.value)
        }
        Ok(())
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
