use super::BuilderV42;
use crate::neo_error::NeoError;
use crate::site_config::SiteConfigV42;
use minijinja::syntax::SyntaxConfig;
use minijinja::Environment;
use std::collections::BTreeMap;

impl BuilderV42<'_> {
    pub fn new_from_site_config(site_config: &SiteConfigV42) -> Result<BuilderV42, NeoError> {
        let mut env = Environment::new();
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );

        Ok(BuilderV42 {
            env,
            errors: vec![],
            errors_v42: vec![],
            generated_pages: BTreeMap::new(),
            page_errors: vec![],
            pages: vec![],
            pages_v42: BTreeMap::new(),
            site_config: site_config.clone(),
            site: None,
            templates: BTreeMap::new(),
            template_load_errors: vec![],
        })
    }
}
