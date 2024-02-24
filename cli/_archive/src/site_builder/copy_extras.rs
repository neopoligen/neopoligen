use crate::site_builder::SiteBuilder;
use fs_extra::dir::copy;
use tracing::instrument;

impl SiteBuilder<'_> {
    #[instrument]
    pub fn copy_extras(&mut self) {
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        options.content_only = true;

        let extras_dir = self.config.folders.site_extras_root.display().to_string();
        let site_output_root_dir = self.config.folders.site_output_root.display().to_string();
        match copy(extras_dir, site_output_root_dir, &options) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }
}
