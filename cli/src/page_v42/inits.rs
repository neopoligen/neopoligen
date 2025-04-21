use super::PageV42b;
use std::path::PathBuf;
// use tracing::{event, instrument, Level};

impl PageV42b {
    //#[instrument(skip(source))]
    pub fn new_from_str_v42(
        source_path: &PathBuf,
        source: &str,
        project_folder: &PathBuf,
    ) -> PageV42b {
        let source_display_path = source_path
            .strip_prefix(project_folder.parent().unwrap())
            .unwrap()
            .to_path_buf();
        PageV42b {
            ast: None,
            cache_hash: None,
            href: None,
            id: None,
            issues: vec![],
            language: None,
            output: None,
            output_for_rss: None,
            parsing_issue: None,
            rel_output_path: None,
            rss_date: None,
            rss_id: None,
            sections: None,
            source: source.to_string(),
            source_absolute_path: source_path.clone(),
            source_display_path,
            status: None,
            title: None,
            title_as_spans: None,
        }
    }
}
