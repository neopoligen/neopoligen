use crate::child::Child;
use crate::config::Config;
use crate::json_plugin::JsonPlugin;
use crate::list::Item;
use crate::list::List;
use crate::page::Page;
use crate::section::Section;
use crate::section_category::SectionCategory;
use crate::span::Span;
use crate::text_plugin::TextPlugin;
use minijinja::Value;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

impl Page {
    pub fn test_with_output_to_root_index_html() -> Page {
        let mut key_value_attributes_for_metadata = BTreeMap::new();
        key_value_attributes_for_metadata
            .insert("date".to_string(), "2024-01-02 03:04:05".to_string());
        key_value_attributes_for_metadata.insert("id".to_string(), "id12345b".to_string());
        key_value_attributes_for_metadata.insert("type".to_string(), "builder-test".to_string());
        key_value_attributes_for_metadata.insert("status".to_string(), "scratch".to_string());
        key_value_attributes_for_metadata.insert("path".to_string(), "/".to_string());
        let de = serde_yaml::Deserializer::from_str("- romeo\n- sierra\n- hotel");
        let yaml_target = match Value::deserialize(de) {
            Ok(data) => {
                if data == Value::from(()) {
                    None
                } else {
                    Some(data)
                }
            }
            Err(_e) => None,
        };

        Page {
            site: None,
            config: Config::mock_basic_config(),
            source_path: PathBuf::from("some-project-root/pages/delta/echo/echo.neo"),
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
                                text: "HomePage".to_string(),
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
                    // attributes: vec![SectionAttribute::Bool {
                    //     key: "rust".to_string(),
                    // }],
                    bounds: "full".to_string(),
                    category: SectionCategory::PreformattedSectionFull {
                        text: Some("preformatted text here".to_string()),
                    },
                    template: "default".to_string(),
                    r#type: "code".to_string(),
                    source: "".to_string(),
                }),
                Child::List(List {
                    r#type: "notes".to_string(),
                    bounds: "full".to_string(),
                    template: "default".to_string(),
                    key_value_attributes: BTreeMap::new(),
                    flag_attributes: BTreeSet::new(),
                    source: "".to_string(),
                    items: vec![
                        Item::ListItem {
                            containers: vec![Child::Block(vec![
                                Span::Word {
                                    text: "whiskey".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                                Span::Space {
                                    text: " ".to_string(),
                                    template: "spans/space.jinja".to_string(),
                                },
                                Span::Word {
                                    text: "papa".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                                Span::Space {
                                    text: "\n".to_string(),
                                    template: "spans/space.jinja".to_string(),
                                },
                                Span::Word {
                                    text: "lima".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                            ])],
                        },
                        Item::ListItem {
                            containers: vec![Child::Block(vec![
                                Span::Word {
                                    text: "xray".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                                Span::Space {
                                    text: " ".to_string(),
                                    template: "spans/space.jinja".to_string(),
                                },
                                Span::Word {
                                    text: "victor".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                                Span::Space {
                                    text: "\n".to_string(),
                                    template: "spans/space.jinja".to_string(),
                                },
                                Span::Word {
                                    text: "romeo".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                            ])],
                        },
                    ],
                }),
                Child::List(List {
                    r#type: "todo".to_string(),
                    bounds: "full".to_string(),
                    template: "default".to_string(),
                    key_value_attributes: BTreeMap::new(),
                    flag_attributes: BTreeSet::new(),
                    source: "".to_string(),
                    items: vec![
                        Item::ChecklistItem {
                            status: None,
                            containers: vec![Child::Block(vec![
                                Span::Word {
                                    text: "alfa".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                                Span::Space {
                                    text: "\n".to_string(),
                                    template: "spans/space.jinja".to_string(),
                                },
                                Span::Word {
                                    text: "charlie".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                            ])],
                        },
                        Item::ChecklistItem {
                            status: Some("x".to_string()),
                            containers: vec![Child::Block(vec![Span::Word {
                                text: "mike".to_string(),
                                template: "spans/word.jinja".to_string(),
                            }])],
                        },
                    ],
                }),
                Child::Section(Section {
                    key_value_attributes: BTreeMap::new(),
                    flag_attributes: BTreeSet::new(),
                    // attributes: vec![],
                    bounds: "full".to_string(),
                    category: SectionCategory::YamlSectionFull {
                        object: yaml_target,
                    },
                    template: "default".to_string(),
                    r#type: "yaml_example".to_string(),
                    source: "".to_string(),
                }),
                Child::TextPlugin(TextPlugin {
                    attributes: vec![],
                    bounds: "full".to_string(),
                    template: "default".to_string(),
                    text: Some("#1b3184".to_string()),
                    r#type: "text_plugin_test".to_string(),
                }),
                Child::JsonPlugin(JsonPlugin {
                    key_value_attributes: BTreeMap::new(),
                    flag_attributes: BTreeSet::new(),
                    bounds: "full".to_string(),
                    template: "default".to_string(),
                    object: Some(
                        serde_json::from_str::<Value>("{ \"color\": \"#bbrrtt\" }").unwrap(),
                    ),
                    r#type: "json_plugin_test".to_string(),
                }),
                Child::Section(Section {
                    key_value_attributes: BTreeMap::new(),
                    flag_attributes: BTreeSet::new(),
                    // attributes: vec![],
                    bounds: "full".to_string(),
                    category: SectionCategory::CommentSectionFull { containers: vec![] },
                    template: "default".to_string(),
                    r#type: "comment".to_string(),
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
