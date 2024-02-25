use crate::child::child;
use crate::child::Child;
use crate::config::Config;
use nom::multi::many1;
use nom::IResult;

pub fn ast<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Vec<Child>> {
    let (unparsed, ast) = many1(|src| child(src, config))(source)?;
    Ok((unparsed, ast))
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::page::Page;
    use crate::section::Section;
    use crate::section_category::SectionCategory;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    #[test]
    fn ast_basic_test() {
        let page = Page::s2_index();
        let mut metadata_key_value_attrs = BTreeMap::new();
        metadata_key_value_attrs.insert("date".to_string(), "2024-02-24 19:11:09".to_string());
        metadata_key_value_attrs.insert("id".to_string(), "site1_index".to_string());
        metadata_key_value_attrs.insert("path".to_string(), "/".to_string());
        let left = vec![
            Child::Section(Section {
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                bounds: "full".to_string(),
                category: SectionCategory::StandardSectionFull {
                    containers: vec![
                        Child::Block(vec![
                            Span::Word {
                                text: "Site".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                            Span::Space {
                                text: " ".to_string(),
                                template: "spans/space.jinja".to_string(),
                            },
                            Span::Word {
                                text: "1".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                            Span::Space {
                                text: " ".to_string(),
                                template: "spans/space.jinja".to_string(),
                            },
                            Span::Word {
                                text: "Home".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                            Span::Space {
                                text: " ".to_string(),
                                template: "spans/space.jinja".to_string(),
                            },
                            Span::Word {
                                text: "Page".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                        ]),
                        Child::Block(vec![
                            Span::Word {
                                text: "The".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                            Span::Space {
                                text: " ".to_string(),
                                template: "spans/space.jinja".to_string(),
                            },
                            Span::Word {
                                text: "initial".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                            Span::Space {
                                text: " ".to_string(),
                                template: "spans/space.jinja".to_string(),
                            },
                            Span::Word {
                                text: "test".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                            Span::Space {
                                text: " ".to_string(),
                                template: "spans/space.jinja".to_string(),
                            },
                            Span::Word {
                                text: "page".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                        ]),
                    ],
                },
                template: "default".to_string(),
                r#type: "title".to_string(),
                source: "-- title\n\nSite 1 Home Page\n\nThe initial test page".to_string(),
            }),
            Child::Section(Section {
                key_value_attributes: metadata_key_value_attrs,
                flag_attributes: BTreeSet::new(),
                bounds: "full".to_string(),
                category: SectionCategory::JsonSectionFull { object: None },
                template: "default".to_string(),
                r#type: "metadata".to_string(),
                source: "-- metadata\n-- date: 2024-02-24 19:11:09\n-- id: site1_index\n-- path: /"
                    .to_string(),
            }),
        ];
        let right = page.ast;
        assert_eq!(left, right);
    }

    // TODO: Put versions of these original
    // tests back in place

    // #[test]
    // // #[ignore]
    // fn start_end_code_section_test() {
    //     let source = "-- code/\n\nCharlie Bravo\n\n-- /code\n\nalfa bravo\n\n-- image";
    //     let config = Config::mock_basic_config();
    //     let left = Ok((
    //         "-- image",
    //         vec![
    //             Child::Section(Section {
    //                 key_value_attributes: BTreeMap::new(),
    //                 flag_attributes: BTreeSet::new(),
    //                 bounds: "start".to_string(),
    //                 category: SectionCategory::PreformattedSectionFull {
    //                     text: Some("Charlie Bravo".to_string()),
    //                 },
    //                 template: "default".to_string(),
    //                 r#type: "code".to_string(),
    //                 source: "-- code/\n\nCharlie Bravo".to_string(),
    //             }),
    //             Child::Section(Section {
    //                 key_value_attributes: BTreeMap::new(),
    //                 flag_attributes: BTreeSet::new(),
    //                 bounds: "end".to_string(),
    //                 category: SectionCategory::PreformattedSectionEnd {
    //                     containers: vec![Child::Block(vec![
    //                         Span::Word {
    //                             text: "alfa".to_string(),
    //                             template: "spans/word.jinja".to_string(),
    //                         },
    //                         Span::Space {
    //                             text: " ".to_string(),
    //                             template: "spans/space.jinja".to_string(),
    //                         },
    //                         Span::Word {
    //                             text: "bravo".to_string(),
    //                             template: "spans/word.jinja".to_string(),
    //                         },
    //                     ])],
    //                 },
    //                 template: "default".to_string(),
    //                 r#type: "code".to_string(),
    //                 source: "-- /code\n\nalfa bravo".to_string(),
    //             }),
    //         ],
    //     ));
    //     let right = ast(source, &config);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // // #[ignore]
    // fn start_end_code_followed_by_start_end() {
    //     let source = "-- code/\n\nRomeo Alfa\n\n-- /code\n\n-- code/\n\nJuliette Hotel\n\n-- /code\n\nYankee\n\n-- div";
    //     let config = Config::mock_basic_config();
    //     let left = Ok((
    //         "-- div",
    //         vec![
    //             Child::Section(Section {
    //                 key_value_attributes: BTreeMap::new(),
    //                 flag_attributes: BTreeSet::new(),
    //                 bounds: "start".to_string(),
    //                 category: SectionCategory::PreformattedSectionFull {
    //                     text: Some("Romeo Alfa".to_string()),
    //                 },
    //                 template: "default".to_string(),
    //                 r#type: "code".to_string(),
    //                 source: "-- code/\n\nRomeo Alfa".to_string(),
    //             }),
    //             Child::Section(Section {
    //                 key_value_attributes: BTreeMap::new(),
    //                 flag_attributes: BTreeSet::new(),
    //                 bounds: "end".to_string(),
    //                 category: SectionCategory::PreformattedSectionEnd { containers: vec![] },
    //                 template: "default".to_string(),
    //                 r#type: "code".to_string(),
    //                 source: "-- /code".to_string(),
    //             }),
    //             Child::Section(Section {
    //                 key_value_attributes: BTreeMap::new(),
    //                 flag_attributes: BTreeSet::new(),
    //                 bounds: "start".to_string(),
    //                 category: SectionCategory::PreformattedSectionFull {
    //                     text: Some("Juliette Hotel".to_string()),
    //                 },
    //                 template: "default".to_string(),
    //                 r#type: "code".to_string(),
    //                 source: "-- code/\n\nJuliette Hotel".to_string(),
    //             }),
    //             Child::Section(Section {
    //                 key_value_attributes: BTreeMap::new(),
    //                 flag_attributes: BTreeSet::new(),
    //                 bounds: "end".to_string(),
    //                 category: SectionCategory::PreformattedSectionEnd {
    //                     containers: vec![Child::Block(vec![Span::Word {
    //                         text: "Yankee".to_string(),
    //                         template: "spans/word.jinja".to_string(),
    //                     }])],
    //                 },
    //                 template: "default".to_string(),
    //                 r#type: "code".to_string(),
    //                 source: "-- /code\n\nYankee".to_string(),
    //             }),
    //         ],
    //     ));
    //     let right = ast(source, &config);
    //     assert_eq!(left, right);
    // }

    //     #[test]
    //     // #[ignore]
    //     fn start_end_standard_section_test() {
    //         let source = "-- /div\n\nCharlie Bravo\n\n-- hr";
    //         let config = Config::mock_basic_config();
    //         let left = Ok((
    //             "-- hr",
    //             vec![
    //                 Child::Section(Section {
    //                     key_value_attributes: BTreeMap::new(),
    //                     flag_attributes: BTreeSet::new(),
    //                     bounds: "end".to_string(),
    //                     category: SectionCategory::StandardSectionEnd {
    //                         containers: vec![Child::Block(vec![
    //                             Span::Word {
    //                                 text: "Charlie".to_string(),
    //                                 template: "spans/word.jinja".to_string(),
    //                             },
    //                             Span::Space {
    //                                 text: " ".to_string(),
    //                                 template: "spans/space.jinja".to_string(),
    //                             },
    //                             Span::Word {
    //                                 text: "Bravo".to_string(),
    //                                 template: "spans/word.jinja".to_string(),
    //                             },
    //                         ])],
    //                     },
    //                     template: "default".to_string(),
    //                     r#type: "div".to_string(),
    //                     source: "-- /div\n\nCharlie Bravo".to_string(),
    //                 }),
    //             ],
    //         ));
    //         let right = ast(source, &config);
    //         assert_eq!(left, right);
    //     }
}
