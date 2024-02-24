// THIS IS DEPCREATED. Remove it when _v2 is done
//
use crate::child::Child;
use crate::page::Page;
use minijinja::value::ValueKind;
use minijinja::Value;

impl Page {
    pub fn place_only(&self, args: &[Value]) -> Vec<Child> {
        match &args[0].kind() {
            ValueKind::String => self
                .ast
                .clone()
                .into_iter()
                .filter_map(|child| match child.clone() {
                    Child::Section(section) => {
                        if args.contains(&Value::from(section.r#type)) {
                            Some(child)
                        } else {
                            None
                        }
                    }
                    _ => None,
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
                                Some(child)
                            } else {
                                None
                            }
                        } else {
                            None
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
    fn place_only_test_with_vec() {
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
                            text: "Foxtrot".to_string(),
                            template: "spans/word.jinja".to_string(),
                        },
                    ])],
                },
                template: "default".to_string(),
                r#type: "title".to_string(),
                source: "".to_string(),
            }),
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
        ];
        let right = p.place_only(&[Value::from(vec!["title".to_string(), "p".to_string()])]);
        assert_eq!(left, right);
    }

    #[test]
    // #[ignore]
    fn place_only_test_with_string() {
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
        let right = p.place_only(&[Value::from("title".to_string())]);
        assert_eq!(left, right);
    }
}
