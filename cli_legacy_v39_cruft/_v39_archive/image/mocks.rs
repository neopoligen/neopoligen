use crate::image::Image;
use std::path::PathBuf;

impl Image {
    pub fn mock_1() -> Image {
        Image {
            alt_text: None,
            alt_text_extended: None,
            height: None,
            source_path: PathBuf::from("/mock/root/images/Some Path/Image Name.JPG"),
            versions: vec![],
            width: None,
        }
    }
}
