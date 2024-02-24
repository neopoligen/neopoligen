use crate::child::Child;
use crate::page::Page;
use minijinja::value::ValueKind;
use minijinja::Value;

impl Page {
    pub fn place_everything_except(&self, args: &[Value]) -> Vec<Child> {
        match &args[0].kind() {
            ValueKind::String => self
                .ast
                .clone()
                .into_iter()
                .filter_map(|child| match child.clone() {
                    Child::Section(section) => {
                        if args.contains(&Value::from(section.r#type)) {
                            None
                        } else {
                            Some(child)
                        }
                    }
                    _ => Some(child),
                })
                .collect(),
            _ => {
                let sections_to_allow: &Vec<_> = &args[0]
                    .to_owned()
                    .try_iter()
                    .expect("pulling args")
                    .collect();
                self.ast
                    .clone()
                    .into_iter()
                    .filter_map(|child| {
                        if let Child::Section(sec) = &child {
                            if sections_to_allow.contains(&sec.r#type.clone().into()) {
                                None
                            } else {
                                Some(child)
                            }
                        } else {
                            Some(child)
                        }
                    })
                    .collect()
            }
        }
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
    // #[ignore]
    fn place_everything_except_with_vec() {
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
                        text: "Foxtrot".to_string(),
                        template: "spans/word.jinja".to_string(),
                    },
                ])],
            },
            template: "default".to_string(),
            r#type: "title".to_string(),
            source: "".to_string(),
        })];
        let right = p
            .place_everything_except(&[Value::from(vec!["p".to_string(), "metadata".to_string()])]);
        assert_eq!(left, right);
    }

    #[test]
    // #[ignore]
    fn place_everything_except_with_string() {
        let mut key_value_attributes_for_metadata = BTreeMap::new();
        key_value_attributes_for_metadata
            .insert("date".to_string(), "2024-01-02 03:04:05".to_string());
        key_value_attributes_for_metadata.insert("id".to_string(), "id12345e".to_string());
        key_value_attributes_for_metadata.insert("type".to_string(), "main-body-test".to_string());
        key_value_attributes_for_metadata.insert("status".to_string(), "draft".to_string());
        let p = Page::test_with_title_metadata_and_one_p();
        let left: Vec<Child> = vec![
            Child::Section(Section {
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
            }),
            Child::Section(Section {
                key_value_attributes: key_value_attributes_for_metadata,
                flag_attributes: BTreeSet::new(),
                bounds: "full".to_string(),
                category: SectionCategory::JsonSectionFull { object: None },
                template: "default".to_string(),
                r#type: "metadata".to_string(),
                source: "".to_string(),
            }),
        ];
        let right = p.place_everything_except(&[Value::from("title".to_string())]);
        assert_eq!(left, right);
    }
}
