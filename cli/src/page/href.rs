use crate::child::Child;
use crate::page::Page;

// This is the relative URL for the page
// from the site root including the
// query string parameters

impl Page {
    pub fn href(&self) -> Option<String> {
        if let Some(path) = self.ast.iter().find_map(|child| {
            if let Child::Section(section) = child {
                let section_type = &section.r#type;
                if section_type == "metadata" {
                    section
                        .key_value_attributes
                        .get("path")
                        .map(|value| value.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        }) {
            Some(path)
        } else {
            if let Some(id) = self.id() {
                match self.title_for_url() {
                    Some(url_title) => Some(format!(
                        "/{}/{}/?{}",
                        &self.config.default_language, id, url_title
                    )),
                    None => Some(format!("/{}/{}/", &self.config.default_language, id)),
                }
            } else {
                Some("/".to_string())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn href_test_basic() {
        let p = Page::id12345d_title_and_metadata();
        let left = Some("/en/id12345d/?bravo-charlie".to_string());
        let right = p.href();
        assert_eq!(left, right);
    }

    #[test]
    fn href_test_with_metadata_path() {
        let p = Page::test_with_output_path();
        let left = Some("/some-path/".to_string());
        let right = p.href();
        assert_eq!(left, right);
    }

    #[test]
    fn href_test_with_path_to_root() {
        let p = Page::test_with_output_to_root_index_html();
        let left = Some("/".to_string());
        let right = p.href();
        assert_eq!(left, right);
    }
}
