use crate::child::Child;
use crate::page::Page;

impl Page {
    pub fn status(&self) -> Option<String> {
        if let Some(the_type) = self.ast.iter().find_map(|child| {
            if let Child::Section(section) = child {
                let section_type = &section.r#type;
                if section_type == "metadata" {
                    section
                        .key_value_attributes
                        .get("status")
                        .map(|value| value.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        }) {
            Some(the_type)
        } else {
            // Note: can't do "unpublished" here
            // it would make it look like you could
            // prevent the page from being published
            // which is not what happens
            Some("published".to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn page_has_id() {
        let p = Page::id12345d_title_and_metadata();
        let left = Some("scratch".to_string());
        let right = p.status();
        assert_eq!(left, right);
    }

    #[test]
    fn return_default_post_if_there_is_no_type() {
        let p = Page::test_with_no_content();
        let left = Some("published".to_string());
        let right = p.status();
        assert_eq!(left, right);
    }
}
