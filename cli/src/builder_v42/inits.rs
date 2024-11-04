use super::BuilderV42;
use crate::neo_error::NeoError;
use crate::neo_error_v42::NeoErrorV42;
use crate::site_config::SiteConfigV42;
use include_dir::{include_dir, Dir};
use minijinja::syntax::SyntaxConfig;
use minijinja::Environment;
use std::collections::BTreeMap;
use std::path::Path;
use tracing::{event, instrument, Level};

impl BuilderV42<'_> {
    #[instrument(skip(site_config))]
    pub fn new_from_site_config(site_config: &SiteConfigV42) -> Result<BuilderV42, NeoError> {
        let mut env = Environment::new();
        let mut template_load_errors = vec![];
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );

        static BUILT_IN_TEMPLATE_DIR: Dir = include_dir!("src/built_in_templates");
        let glob = "**/*.neoj";
        for entry in BUILT_IN_TEMPLATE_DIR.find(glob).unwrap() {
            // dbg!("Found {}", entry.path().display());

            let data = BUILT_IN_TEMPLATE_DIR.get_file(entry.path());
            match env.add_template_owned(
                format!("built-in/{}", entry.path().display().to_string()),
                data.expect(
                    format!(
                        "Could not open built-in include: {}",
                        entry.path().display().to_string()
                    )
                    .as_str(),
                )
                .contents_utf8()
                .unwrap()
                .to_string(),
            ) {
                Ok(()) => (),
                Err(e) => {
                    template_load_errors.push(NeoErrorV42::TemplateLoadError {
                        message: e.to_string(),
                        details: e.display_debug_info().to_string(),
                    });
                    event!(Level::ERROR, "{}", e.display_debug_info().to_string());
                    ()
                }
            }
        }

        // let built_in_tempates_list = vec![
        //     "blocks/paragraph",
        //     "css-output",
        //     "generic-basic",
        //     "generic-css",
        //     "generic-end",
        //     "generic-html",
        //     "generic-json5",
        //     "generic-list-item-paragraph",
        //     "generic-list-item-single",
        //     "generic-list-item",
        //     "generic-list",
        //     "generic-raw",
        //     "generic-script",
        //     "helpers",
        //     "pages-default-post",
        //     "pages-problems-index-page",
        //     "pages-issue-page",
        //     "script-output",
        //     "span-text",
        //     "template-error",
        // ];

        // built_in_tempates_list.iter().for_each(|name| {
        //     let data = BUILT_IN_TEMPLATE_DIR.get_file(format!("{}.neoj", name));
        //     match env.add_template_owned(
        //         format!("built-in/{}.neoj", name),
        //         data.expect(format!("Could not open built-in include: {}", name).as_str())
        //             .contents_utf8()
        //             .unwrap()
        //             .to_string(),
        //     ) {
        //         Ok(()) => (),
        //         Err(e) => {
        //             template_load_errors.push(NeoErrorV42::TemplateLoadError {
        //                 message: e.to_string(),
        //                 details: e.display_debug_info().to_string(),
        //             });
        //             event!(Level::ERROR, "{}", e.display_debug_info().to_string());
        //             ()
        //         }
        //     }
        // });

        Ok(BuilderV42 {
            env,
            // errors: vec![],
            errors_v42: vec![],
            //generated_pages: BTreeMap::new(),
            page_errors: vec![],
            // pages: vec![],
            pages_v42: BTreeMap::new(),
            site_config: site_config.clone(),
            // site: None,
            site_v42b: None,
            templates: BTreeMap::new(),
            template_load_errors,
        })
    }
}
