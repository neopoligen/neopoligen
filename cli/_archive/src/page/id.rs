use crate::child::Child;
use crate::page::Page;

impl Page {
    pub fn id(&self) -> Option<String> {
        self.ast.iter().find_map(|child| {
            if let Child::Section(section) = child {
                let section_type = &section.r#type;
                if section_type == "metadata" {
                    // dbg!("---------------", &section.key_value_attributes, "--------------");
                    section
                        .key_value_attributes
                        .get("id")
                        .map(|value| value.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn page_has_id() {
        let p = Page::id12345d_title_and_metadata();
        let left = Some("id12345d".to_string());
        let right = p.id();
        assert_eq!(left, right);
    }

    #[test]
    fn return_none_if_there_is_no_id() {
        let p = Page::test_with_no_content();
        let left = None;
        let right = p.id();
        assert_eq!(left, right);
    }
}
