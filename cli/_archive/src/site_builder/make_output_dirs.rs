use crate::helpers::make_parent_folder::make_parent_folder;
use crate::site_builder::SiteBuilder;

impl SiteBuilder<'_> {
    pub fn make_output_dirs(&self) {
        println!("Making output directories");
        for (_, page) in self.site.pages.iter() {
            if let Some(output_path) = &page.output_path() {
                let _ = make_parent_folder(output_path).unwrap();
            }
        }
    }
}
