use crate::page::Page;
// use minijinja::Value;

// TOOD: Creates a list of the folder in the
// files path from the base of the content directory

impl Page {
    pub fn folders(&self) -> Vec<String> {
        self.source_path()
            .strip_prefix(self.config.folders.site_production_content_root.clone())
            .unwrap()
            .parent()
            .unwrap()
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn filter_basic_test() {
        let p = Page::test_with_output_to_root_index_html();
        let left: Vec<String> = ["delta".to_string(), "echo".to_string()]
            .into_iter()
            .collect();
        let right = p.folders();
        assert_eq!(left, right);
    }
}
