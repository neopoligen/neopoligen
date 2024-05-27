use crate::image::Image;
use std::path::PathBuf;

impl Image {
    pub fn mock_1() -> Image {
        Image {
            source_path: PathBuf::from("/mock/root/images/Some Path/Image Name.JPG"),
            width: None,
            height: None,
            versions: vec![],
        }
    }
}
