use crate::child::Child;
use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn output_path(&self) -> Option<PathBuf> {
        let mut output_path = self.config.folders.site_output_root.clone();
        if let Some(path) = self.ast.iter().find_map(|child| {
            if let Child::Section(section) = child {
                let section_type = &section.r#type;
                if section_type == "metadata" {
                    if let Some(value) = section.key_value_attributes.get("path") {
                        let url_dir_path = PathBuf::from(value.to_string());
                        output_path.push(url_dir_path.strip_prefix("/").unwrap());
                        output_path.push("index.html");
                        Some(output_path.clone())
                    } else {
                        None
                    }
                    // if let SectionAttribute::KeyValue { key, value } = attr {
                    //     if key == "path" {
                    //         let url_dir_path = PathBuf::from(value.to_string());
                    //         output_path.push(url_dir_path.strip_prefix("/").unwrap());
                    //         output_path.push("index.html");
                    //         Some(output_path.clone())
                    //     } else {
                    //         None
                    //     }
                    // } else {
                    //     None
                    // }
                } else {
                    None
                }
            } else {
                None
            }
        }) {
            Some(path)
        } else {
            output_path.push(&self.config.default_language);
            if let Some(id) = self.id() {
                output_path.push(id);
                output_path.push("index.html");
                Some(output_path.clone())
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn can_get_page_path() {
        let p = Page::id12345d_title_and_metadata();
        let left = Some(PathBuf::from(
            "some-project-root/docs/en/id12345d/index.html",
        ));
        let right = p.output_path();
        assert_eq!(left, right);
    }

    #[test]
    fn can_not_get_page_path() {
        let p = Page::test_with_no_content();
        let left = None;
        let right = p.output_path();
        assert_eq!(left, right);
    }

    #[test]
    // #[ignore]
    fn get_metadata_override_path() {
        let p = Page::test_with_output_path();
        let left = Some(PathBuf::from("some-project-root/docs/some-path/index.html"));
        let right = p.output_path();
        assert_eq!(left, right);
    }

    #[test]
    // #[ignore]
    fn check_root_path() {
        let p = Page::test_with_output_to_root_index_html();
        let left = Some(PathBuf::from("some-project-root/docs/index.html"));
        let right = p.output_path();
        assert_eq!(left, right);
    }
}
