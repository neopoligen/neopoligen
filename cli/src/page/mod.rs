pub mod builders;
pub mod filters;
pub mod folders;
pub mod full_title;
pub mod full_title_for_page;
pub mod has_filter;
pub mod href;
pub mod id;
pub mod link_or_title;
pub mod link_or_title_filtered;
pub mod main_body;
pub mod metadata;
pub mod new;
pub mod output_path;
pub mod parse;
pub mod place_everything;
pub mod place_everything_except;
pub mod place_only;
pub mod source;
pub mod status;
pub mod tags;
pub mod title_for_url;
pub mod r#type;
pub mod url_path;
pub mod url_for_page;

use crate::child::*;
use crate::config::Config;
use serde::Serialize;
use std::collections::BTreeSet;
use std::path::PathBuf;
use crate::site::Site;
// use serde_json::Value as SerdeValue;

// Struct for a reminder:
//
// Page
//   - new
//      - parse
//          - ast
//              - child
//                  - section
//                      - section_*
//                          - section_attribute
//                              -- section_attribute_*
//                  - item
//                  - block

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    pub ast: Vec<Child>,
    pub config: Config,
    pub source: String,
    pub source_path: PathBuf,
    // pub folder_path: PathBuf,
    pub site: Option<Site>
}

impl Page {

    pub fn children(&self) -> Vec<String> {
        if let Some(cf) = &self.metadata().get("children") {
            if let Ok(path_parts) = serde_json::from_str::<Vec<String>>(cf.as_str()) {
                path_parts
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    pub fn filter_set(&self) -> BTreeSet<String> {
        // TODO: deprecate .filter() and just use this directly
        let mut filter_set = BTreeSet::new();
        self.filters().iter().for_each(|f|{  filter_set.insert(f.to_string()); });
        filter_set
    }

    pub fn path_parts(&self) -> Vec<String> {
        self.source_path().strip_prefix(&self.config.folders.site_production_content_root.clone()).unwrap()
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string().to_lowercase())
            .collect()
    }


    pub fn section(&self, section_type: String) -> Vec<Child> {
        // Needs tests
        self.ast
            .clone()
            .into_iter()
            .filter_map(|child| {
                if let Child::Section(sec) = &child {
                    if sec.r#type == section_type {
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

    pub fn section_by_id(&self, section_id: String) -> Vec<Child> {
        // Tests are in the neopoligen-site theme
        // TODO: Add explicit tests here too
        self.ast
            .clone()
            .into_iter()
            .filter_map(|child| {
                match child.clone(){
                    Child::Section(sec) => {
                        match sec.key_value_attributes.get("id") {
                            Some(id) => {if id.to_string() == section_id {
                                Some(child)
                            } else {
                                None}
                            },
                            None => None
                        }
                    } ,
                    Child::List(sec) => {
                        match sec.key_value_attributes.get("id") {
                            Some(id) => {if id.to_string() == section_id {
                                Some(child)
                            } else {
                                None}
                            },
                            None => None
                        }
                    } ,
                    Child::JsonPlugin(sec) => {
                        match sec.key_value_attributes.get("id") {
                            Some(id) => {if id.to_string() == section_id {
                                Some(child)
                            } else {
                                None}
                            },
                            None => None
                        }
                    } ,
                    _ => None
                }
                // if let Child::Section(sec) = &child {
                //     if let Some(id) = sec.key_value_attributes.get("id") {
                //         if id.to_string() == section_id {
                //             Some(child)
                //         }
                //         else {
                //             None
                //         }
                //     } else {
                //         None
                //     }
                // } else {
                //     None
                // }
            })
            .collect()
    }

    pub fn source_path(&self) -> PathBuf {
        self.source_path.clone()
    }

}


// TODO: Move tests to top level test dir so they
// don't break rust-analyzer formatting because
// of the strings (which I think are the problem)

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::ast;
    use crate::config::Config;
    // use crate::json_plugin::JsonPlugin;
    use crate::list::Item;
    use crate::list::List;
    use crate::section::Section;
    use crate::section_category::SectionCategory;
    use crate::span::Span;
    // use crate::text_plugin::TextPlugin;
    use minijinja::Value;
    use pretty_assertions::assert_eq;
    use serde::Deserialize;
    use std::collections::{BTreeMap, BTreeSet};


    #[test]
    fn path_parts_test() {
        let p = Page::level1a_index();
        let left: Vec<String> = vec![
            "level1a".to_string(), 
            "_index.neo".to_string()
        ];
        let right = p.path_parts();
        assert_eq!(left, right);
    }


    #[test]
    // #[ignore]
    fn page_integration() {
        let source = r#"-- title

Title Alfa

-- metadata
-- date: 2024-01-22 03:24:16
-- id: 2bioglon
-- type: post
-- status: draft

"#;
        let config = Config::mock_basic_config();
        let page = Page::new(PathBuf::from("some-project-root/pages/index.neo"), source, config);
        let mut key_value_attributes_for_metadata = BTreeMap::new();
        key_value_attributes_for_metadata.insert("id".to_string(), "2bioglon".to_string());
        key_value_attributes_for_metadata.insert("status".to_string(), "draft".to_string());
        key_value_attributes_for_metadata.insert("type".to_string(), "post".to_string());
        key_value_attributes_for_metadata
            .insert("date".to_string(), "2024-01-22 03:24:16".to_string());
        let metadata_source = r#"-- metadata
-- date: 2024-01-22 03:24:16
-- id: 2bioglon
-- type: post
-- status: draft"#;
        let left = vec![
            Child::Section(Section {
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                bounds: "full".to_string(),
                category: SectionCategory::StandardSectionFull {
                    containers: vec![Child::Block(vec![
                        Span::Word {
                            text: "Title".to_string(),
                            template: "spans/word.jinja".to_string(),
                        },
                        Span::Space {
                            text: " ".to_string(),
                            template: "spans/space.jinja".to_string(),
                        },
                        Span::Word {
                            text: "Alfa".to_string(),
                            template: "spans/word.jinja".to_string(),
                        },
                    ])],
                },
                template: "default".to_string(),
                r#type: "title".to_string(),
                source: "-- title\n\nTitle Alfa".to_string(),
            }),
            Child::Section(Section {
                key_value_attributes: key_value_attributes_for_metadata,
                flag_attributes: BTreeSet::new(),
                bounds: "full".to_string(),
                category: SectionCategory::JsonSectionFull { object: None },
                template: "default".to_string(),
                r#type: "metadata".to_string(),
                source: metadata_source.to_string(),
            }),
        ];
        let right = page.ast;
        assert_eq!(left, right);
    }

    #[test]
    // NOTE: This doesn't have section sources in it yet
    // which is why it's ignored
    fn page_integration2() {
        let source = r#"-- title

Sierra Papa

Alfa <<strong|bravo>>

-- code
-- rust

// this is rust



-- notes

- romeo tango
whiskey

- papa mike
victor

-- todo

[] foxtrot
echo

[x] sierra

-- yaml-example

- tango
- papa
- sierra

-- comment

this will not show up

-- metadata
-- date: 1234-05-06 07:08:09
-- id: aaff22jj
-- type: example
-- status: unpublished

"#
        .to_string();
        let config = Config::mock_basic_config();
        let de = serde_yaml::Deserializer::from_str("- tango\n- papa\n- sierra");
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
        let mut key_value_attributes_for_metadata = BTreeMap::new();
        key_value_attributes_for_metadata.insert("id".to_string(), "aaff22jj".to_string());
        key_value_attributes_for_metadata.insert("status".to_string(), "unpublished".to_string());
        key_value_attributes_for_metadata.insert("type".to_string(), "example".to_string());
        key_value_attributes_for_metadata
            .insert("date".to_string(), "1234-05-06 07:08:09".to_string());
        let mut code_flag_attributes = BTreeSet::new();
        code_flag_attributes.insert("rust".to_string());

        let left = Ok((
            "",
            vec![
                Child::Section(Section {
                    key_value_attributes: BTreeMap::new(),
                    flag_attributes: BTreeSet::new(),
                    bounds: "full".to_string(),
                    category: SectionCategory::StandardSectionFull {
                        containers: vec![
                            Child::Block(vec![
                                        Span::Word {
                                            text: "Sierra".to_string(),
                                            template: "spans/word.jinja".to_string(),
                                        },
                                        Span::Space {
                                            text: " ".to_string(),
                                            template: "spans/space.jinja".to_string(),
                                        },
                                        Span::Word {
                                            text: "Papa".to_string(),
                                            template: "spans/word.jinja".to_string(),
                                        },
                                    ],

                            ),
                            Child::Block(vec![
                                        Span::Word {
                                            text: "Alfa".to_string(),
                                            template: "spans/word.jinja".to_string(),
                                        },
                                        Span::Space {
                                            text: " ".to_string(),
                                            template: "spans/space.jinja".to_string(),
                                        },
                                        Span::StandardSpan { 
                                            key_value_attributes: BTreeMap::new(), 
                                            flag_attributes: BTreeSet::new(), 
                                            span_type: "strong".to_string(), 
                                            spans: vec![
                                                Span::Word {
                                                text: "bravo".to_string(),
                                                template: "spans/word.jinja".to_string(),
                                                },
                                            ], 
                                            template: "spans/strong.jinja".to_string(),
                                        },
                                    ]
                            )],
                    },
                    template: "default".to_string(),
                    r#type: "title".to_string(),
                    source: "-- title\n\nSierra Papa\n\nAlfa <<strong|bravo>>".to_string(),
                }),
                Child::Section(Section {
                    key_value_attributes: BTreeMap::new(),
                    flag_attributes: code_flag_attributes,
                    bounds: "full".to_string(),
                    category: SectionCategory::PreformattedSectionFull {
                        text: Some("// this is rust".to_string()),
                    },
                    template: "default".to_string(),
                    r#type: "code".to_string(),
                    source: "-- code\n-- rust\n\n// this is rust".to_string(),
                }),
                Child::List(List {
                    r#type: "notes".to_string(),
                    bounds: "full".to_string(),
                    template: "default".to_string(),
                    key_value_attributes: BTreeMap::new(),
                    flag_attributes: BTreeSet::new(),
                    source: "-- notes\n\n- romeo tango\nwhiskey\n\n- papa mike\nvictor".to_string(),
                    items: vec![
                        Item::ListItem {
                            containers: vec![Child::Block(vec![
                                Span::Word {
                                    text: "romeo".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                                Span::Space {
                                    text: " ".to_string(),
                                    template: "spans/space.jinja".to_string(),
                                },
                                Span::Word {
                                    text: "tango".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                                Span::Space {
                                    text: "\n".to_string(),
                                    template: "spans/space.jinja".to_string(),
                                },
                                Span::Word {
                                    text: "whiskey".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                            ])],
                        },
                        Item::ListItem {
                            containers: vec![Child::Block(vec![
                                Span::Word {
                                    text: "papa".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                                Span::Space {
                                    text: " ".to_string(),
                                    template: "spans/space.jinja".to_string(),
                                },
                                Span::Word {
                                    text: "mike".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                                Span::Space {
                                    text: "\n".to_string(),
                                    template: "spans/space.jinja".to_string(),
                                },
                                Span::Word {
                                    text: "victor".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                            ])],
                        },
                    ],
                }),
                Child::List(List {
                    key_value_attributes: BTreeMap::new(),
                    flag_attributes: BTreeSet::new(),
                    r#type: "todo".to_string(),
                    bounds: "full".to_string(),
                    template: "default".to_string(),
                    source: "-- todo\n\n[] foxtrot\necho\n\n[x] sierra".to_string(),
                    items: vec![
                        Item::ChecklistItem {
                            status: None,
                            containers: vec![Child::Block(vec![
                                Span::Word {
                                    text: "foxtrot".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                                Span::Space {
                                    text: "\n".to_string(),
                                    template: "spans/space.jinja".to_string(),
                                },
                                Span::Word {
                                    text: "echo".to_string(),
                                    template: "spans/word.jinja".to_string(),
                                },
                            ])],
                        },
                        Item::ChecklistItem {
                            status: Some("x".to_string()),
                            containers: vec![Child::Block(vec![Span::Word {
                                text: "sierra".to_string(),
                                template: "spans/word.jinja".to_string(),
                            }])],
                        },
                    ],
                }),
                Child::Section(Section {
                    key_value_attributes: BTreeMap::new(),
                    flag_attributes: BTreeSet::new(),
                    bounds: "full".to_string(),
                    category: SectionCategory::YamlSectionFull {
                        object: yaml_target,
                    },
                    template: "default".to_string(),
                    r#type: "yaml-example".to_string(),
                    source: "-- yaml-example\n\n- tango\n- papa\n- sierra".to_string(),
                }),
                Child::Section(Section {
                    key_value_attributes: BTreeMap::new(),
                    flag_attributes: BTreeSet::new(),
                    bounds: "full".to_string(),
                    category: SectionCategory::CommentSectionFull { 
                        containers: vec![Child::Block(vec![
                            Span::Word {
                                text: "this".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                            Span::Space {
                                text: " ".to_string(),
                                template: "spans/space.jinja".to_string(),
                            },
                            Span::Word {
                                text: "will".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                            Span::Space {
                                text: " ".to_string(),
                                template: "spans/space.jinja".to_string(),
                            },
                            Span::Word {
                                text: "not".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                            Span::Space {
                                text: " ".to_string(),
                                template: "spans/space.jinja".to_string(),
                            },
                            Span::Word {
                                text: "show".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                            Span::Space {
                                text: " ".to_string(),
                                template: "spans/space.jinja".to_string(),
                            },
                            Span::Word {
                                text: "up".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                        ])],
                    },
                    template: "default".to_string(),
                    r#type: "comment".to_string(),
                    source: "-- comment\n\nthis will not show up".to_string(),
                }),
                Child::Section(Section {
                    key_value_attributes: key_value_attributes_for_metadata,
                    flag_attributes: BTreeSet::new(),
                    bounds: "full".to_string(),
                    category: SectionCategory::JsonSectionFull { object: None },
                    template: "default".to_string(),
                    r#type: "metadata".to_string(),
                    source: "-- metadata\n-- date: 1234-05-06 07:08:09\n-- id: aaff22jj\n-- type: example\n-- status: unpublished".to_string(),
                }),
            ],
        ));
        let right = ast(&source, &config);
        assert_eq!(left, right);
    }
}

