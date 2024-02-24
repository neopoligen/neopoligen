use crate::site_builder::SiteBuilder;
use std::fs;

// Empty the output directory after doing a
// validation check that it's named "docs" to
// help prevent bugs for eating anything else

impl SiteBuilder<'_> {
    pub fn empty_output_dir(&self) {
        if let Some(dir_name) = &self.config.folders.site_output_root.file_name() {
            if dir_name.to_str().unwrap() == "docs" {
                println!(
                    "Emptying: {}",
                    &self.config.folders.site_output_root.display()
                );
                let _ = fs::remove_dir_all(&self.config.folders.site_output_root);
                let _ = fs::create_dir_all(&self.config.folders.site_output_root);
            }
        }
    }
}
