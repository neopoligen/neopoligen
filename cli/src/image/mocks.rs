use crate::image::Image;
use crate::site_config::SiteConfig;
use std::path::PathBuf;

impl Image {
    pub fn mock_1() -> Image {
        Image {
            config: SiteConfig::mock1(),
            source_path: PathBuf::from("/mock/root/images/Some Path/Image Name.JPG"),
            width: Some(1920),
            height: Some(1080),
        }
    }
}
