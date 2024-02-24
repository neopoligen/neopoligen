use crate::site_builder::SiteBuilder;
use fs_extra::copy_items;
use tracing::instrument;

impl SiteBuilder<'_> {
    #[instrument]
    pub fn copy_assets(&mut self) {
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        let from_paths = vec![format!(
            "{}/{}",
            self.config.folders.theme_root.display(),
            "theme-assets"
        )];
        let output_styles_folder = format!("{}", self.config.folders.site_output_root.display());
        match copy_items(&from_paths, output_styles_folder, &options) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }
}
