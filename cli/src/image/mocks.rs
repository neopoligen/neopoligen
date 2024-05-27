use std::path::PathBuf;

use crate::image::Image;

impl Image {
    pub fn mock_1() -> Image {
        Image {
            source_path: PathBuf::from("/mock/root/images/Some Path/Image Name.jpg"),
            source_root: PathBuf::from("/mock/root/images"),
            width: Some(1920),
            height: Some(1080),
        }
    }
}
