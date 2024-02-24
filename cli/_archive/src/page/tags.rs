use crate::child::Child;
use crate::page::Page;

impl Page {
    pub fn tags(&self) -> Vec<String> {
        if let Some(tags) = self.ast.iter().find_map(|child| {
            if let Child::Section(section) = child {
                let section_type = &section.r#type;
                if section_type == "tags" {
                    Some(
                        section
                            .flag_attributes
                            .iter()
                            .map(|flag| flag.to_string().to_lowercase())
                            .collect::<Vec<String>>(),
                    )
                } else {
                    None
                }
            } else {
                None
            }
        }) {
            tags
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn page_tag_test_with_tags() {
        let p = Page::id12345c_tags();
        let left: Vec<String> = vec![
            "example_tag_alfa".to_string(),
            "example_tag_bravo".to_string(),
        ];
        let right = p.tags();
        assert_eq!(left, right);
    }

    #[test]
    fn page_tag_test_without_tags() {
        let p = Page::test_with_no_content();
        let left: Vec<String> = vec![];
        let right = p.tags();
        assert_eq!(left, right);
    }
}
