// DEPREATED: Remove when file specifif teampltes are
// in place

use crate::{
    helpers::get_file_paths_for_extension::get_file_paths_for_extension, site_builder::SiteBuilder,
};
use minijinja::context;
use minijinja::Environment;
use minijinja::Syntax;
use std::fs;
use tracing::instrument;

impl SiteBuilder<'_> {
    #[instrument]
    pub fn load_custom_templates(&mut self) {
        get_file_paths_for_extension(&self.config.folders.theme_custom_sections_root, "jinja")
            .iter()
            .for_each(|input_path| {
                let section_part = input_path
                    .strip_prefix(&self.config.folders.theme_custom_sections_root)
                    .unwrap();
                let prod_template_name = format!("sections/{}", section_part.display());
                self.env.remove_template(prod_template_name.as_str());

                // TODO: Should be able to deprecate this since the
                // classes are built from the values in the data
                let mut skeleton_env = Environment::new();
                skeleton_env
                    .set_syntax(Syntax {
                        block_start: "NA_BLOCK_START".into(),
                        block_end: "NA_BLOCK_END".into(),
                        variable_start: "{$".into(),
                        variable_end: "$}".into(),
                        comment_start: "NA_COMMENT_START".into(),
                        comment_end: "NA_COMMENT_END".into(),
                    })
                    .unwrap();

                skeleton_env.clear_templates();

                let skeleton = fs::read_to_string(input_path).unwrap();
                skeleton_env
                    .add_template_owned("section_template", skeleton)
                    .unwrap();
                let tmpl = skeleton_env.get_template("section_template").unwrap();
                let output = tmpl.render(context!(SECTION_KEY => "")).unwrap();
                self.env
                    .add_template_owned(prod_template_name.to_string(), output)
                    .unwrap();
            });
    }
}
