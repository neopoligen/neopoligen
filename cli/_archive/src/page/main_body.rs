use crate::child::Child;
use crate::page::Page;
use tracing::instrument;

impl Page {
    #[instrument]
    pub fn main_body(&self) -> Vec<Child> {
        self.ast
            .clone()
            .into_iter()
            .filter_map(|child| {
                if let Child::Section(sec) = &child {
                    if self
                        .config
                        .main_body_section_excludes
                        .get(&sec.r#type)
                        .is_some()
                    {
                        None
                    } else {
                        Some(child)
                    }
                } else if let Child::List(sec) = &child {
                    if self
                        .config
                        .main_body_section_excludes
                        .get(&sec.r#type)
                        .is_some()
                    {
                        None
                    } else {
                        Some(child)
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::section::Section;
    use crate::section_category::SectionCategory;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn main_body_excludes_the_right_sections() {
        let p = Page::test_with_title_metadata_and_one_p();
        let left: Vec<Child> = vec![Child::Section(Section {
            key_value_attributes: BTreeMap::new(),
            flag_attributes: BTreeSet::new(),
            bounds: "full".to_string(),
            category: SectionCategory::StandardSectionFull {
                containers: vec![Child::Block(vec![
                    Span::Word {
                        text: "Tango".to_string(),
                        template: "spans/word.jinja".to_string(),
                    },
                    Span::Space {
                        text: " ".to_string(),
                        template: "spans/space.jinja".to_string(),
                    },
                    Span::Word {
                        text: "whiskey".to_string(),
                        template: "spans/word.jinja".to_string(),
                    },
                ])],
            },
            template: "default".to_string(),
            r#type: "p".to_string(),
            source: "".to_string(),
        })];
        let right = p.main_body();
        assert_eq!(left, right);
    }
}
