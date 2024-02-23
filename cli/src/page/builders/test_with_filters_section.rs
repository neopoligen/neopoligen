use crate::child::Child;
use crate::config::Config;
use crate::page::Page;
use crate::section::Section;
use crate::section_category::SectionCategory;
use crate::span::Span;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

impl Page {
    pub fn test_with_filters_section() -> Page {
        let mut key_value_attributes_for_metadata = BTreeMap::new();
        key_value_attributes_for_metadata
            .insert("date".to_string(), "2024-02-07 03:04:05".to_string());
        key_value_attributes_for_metadata.insert("id".to_string(), "id778866".to_string());
        key_value_attributes_for_metadata.insert("type".to_string(), "filter-test".to_string());
        key_value_attributes_for_metadata.insert("status".to_string(), "published".to_string());
        let mut flag_attrs_for_filters_seciton = BTreeSet::new();
        flag_attrs_for_filters_seciton.insert("lima".to_string());
        let mut flag_attrs_for_tags_seciton = BTreeSet::new();
        flag_attrs_for_tags_seciton.insert("HOTEL".to_string());

        Page {
            site: None,
            config: Config::mock_basic_config(),
            source_path: PathBuf::from("some-project-root/pages/with-file.neo"),
            source:
                "This is not really the source for this page, it's just a placeholder for testing"
                    .to_string(),
            ast: vec![
                Child::Section(Section {
                    key_value_attributes: BTreeMap::new(),
                    flag_attributes: BTreeSet::new(),
                    bounds: "full".to_string(),
                    category: SectionCategory::StandardSectionFull {
                        containers: vec![Child::Block(vec![
                            Span::Word {
                                text: "Filter".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                            Span::Space {
                                text: " ".to_string(),
                                template: "spans/space.jinja".to_string(),
                            },
                            Span::Word {
                                text: "Test".to_string(),
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
                Child::Section(Section {
                    key_value_attributes: BTreeMap::new(),
                    flag_attributes: flag_attrs_for_filters_seciton,
                    bounds: "full".to_string(),
                    category: SectionCategory::JsonSectionFull { object: None },
                    template: "default".to_string(),
                    r#type: "filters".to_string(),
                    source: "".to_string(),
                }),
                Child::Section(Section {
                    key_value_attributes: BTreeMap::new(),
                    flag_attributes: flag_attrs_for_tags_seciton,
                    bounds: "full".to_string(),
                    category: SectionCategory::JsonSectionFull { object: None },
                    template: "default".to_string(),
                    r#type: "tags".to_string(),
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
            ],
        }
    }
}
