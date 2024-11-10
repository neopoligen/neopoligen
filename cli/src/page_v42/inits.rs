use super::PageV42b;
use std::path::PathBuf;
use tracing::{event, instrument, Level};

impl PageV42b {
    #[instrument(skip(source))]
    pub fn new_from_str_v42(
        source_path: &PathBuf,
        source: &str,
        project_dir: &PathBuf,
    ) -> PageV42b {
        let source_display_path = source_path
            .strip_prefix(project_dir.parent().unwrap())
            .unwrap()
            .to_path_buf();
        PageV42b {
            ast: None,
            parsing_issue: None,
            href: None,
            id: None,
            issues: vec![],
            language: None,
            output: None,
            rel_output_path: None,
            sections: None,
            source: source.to_string(),
            source_absolute_path: source_path.clone(),
            source_display_path,
            title: None,
            title_as_spans: None,
        }
    }
}
