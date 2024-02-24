use crate::child::child;
use crate::child::Child;
use crate::config::Config;
use crate::section::Section;
use crate::section_category::SectionCategory;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn standard_section_full<'a>(
    source: &'a str,
    r#type: &str,
    key_value_attributes: BTreeMap<String, String>,
    flag_attributes: BTreeSet<String>,
    config: &'a Config,
    initial_source: &str,
) -> IResult<&'a str, Child> {
    if config
        .section_categories
        .standard
        .contains(&r#type.to_string())
    {
        let (source, containers) = many0(|src| child(src, config))(source)?;
        let (source, _) = tuple((space0, line_ending))(source)?;
        let (source, _) = multispace0(source)?;
        let section = Child::Section(Section {
            key_value_attributes,
            flag_attributes,
            bounds: "full".to_string(),
            category: SectionCategory::StandardSectionFull { containers },
            template: "default".to_string(),
            r#type: r#type.to_string(),
            source: initial_source
                .replace(source, "")
                .as_str()
                .trim()
                .to_string(),
        });
        Ok((source, section))
    } else {
        Err(Err::Error(Error::new(source, ErrorKind::TakeUntil)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;

    #[test]
    // #[ignore]
    fn standard_section_full_empty_section() {
        let source = "\n\n-- p\n\n";
        let r#type = "title";
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let config = Config::mock_basic_config();
        let initial_source = "-- title\n\n-- p\n\n";
        let left = Ok((
            "-- p\n\n",
            Child::Section(Section {
                key_value_attributes: key_value_attributes.clone(),
                flag_attributes: flag_attributes.clone(),
                bounds: "full".to_string(),
                category: SectionCategory::StandardSectionFull { containers: vec![] },
                template: "default".to_string(),
                r#type: "title".to_string(),
                source: "-- title".to_string(),
            }),
        ));
        let right = standard_section_full(
            source,
            r#type,
            key_value_attributes,
            flag_attributes,
            &config,
            initial_source,
        );
        assert_eq!(left, right);
    }

    #[test]
    // #[ignore]
    fn standard_section_full_one_paragraph() {
        let initial_source = "-- p\n\ndelta lima\n\n-- hr\n\n";
        let source = "delta lima\n\n-- hr\n\n";
        let r#type = "p";
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let config = Config::mock_basic_config();
        let left = Ok((
            "-- hr\n\n",
            Child::Section(Section {
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                bounds: "full".to_string(),
                category: SectionCategory::StandardSectionFull {
                    containers: vec![Child::Block(vec![
                        Span::Word {
                            text: "delta".to_string(),
                            template: "spans/word.jinja".to_string(),
                        },
                        Span::Space {
                            text: " ".to_string(),
                            template: "spans/space.jinja".to_string(),
                        },
                        Span::Word {
                            text: "lima".to_string(),
                            template: "spans/word.jinja".to_string(),
                        },
                    ])],
                },
                template: "default".to_string(),
                r#type: "p".to_string(),
                source: "-- p\n\ndelta lima".to_string(),
            }),
        ));
        let right = standard_section_full(
            source,
            r#type,
            key_value_attributes,
            flag_attributes,
            &config,
            initial_source,
        );
        assert_eq!(left, right);
    }

    #[test]
    // #[ignore]
    fn standard_section_full_single_paragraph_on_multiple_lines() {
        let source = "tango alfa\nbravo\n\n-- hr\n\n";
        let r#type = "p";
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let initial_source = "-- p\n\ntango alfa\nbravo\n\n-- hr\n\n";
        let config = Config::mock_basic_config();
        let left = Ok((
            "-- hr\n\n",
            Child::Section(Section {
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                bounds: "full".to_string(),
                category: SectionCategory::StandardSectionFull {
                    containers: vec![Child::Block(vec![
                        Span::Word {
                            text: "tango".to_string(),
                            template: "spans/word.jinja".to_string(),
                        },
                        Span::Space {
                            text: " ".to_string(),
                            template: "spans/space.jinja".to_string(),
                        },
                        Span::Word {
                            text: "alfa".to_string(),
                            template: "spans/word.jinja".to_string(),
                        },
                        Span::Space {
                            text: "\n".to_string(),
                            template: "spans/space.jinja".to_string(),
                        },
                        Span::Word {
                            text: "bravo".to_string(),
                            template: "spans/word.jinja".to_string(),
                        },
                    ])],
                },
                template: "default".to_string(),
                r#type: "p".to_string(),
                source: "-- p\n\ntango alfa\nbravo".to_string(),
            }),
        ));
        let right = standard_section_full(
            source,
            r#type,
            key_value_attributes,
            flag_attributes,
            &config,
            initial_source,
        );
        assert_eq!(left, right);
    }

    #[test]
    // #[ignore]
    fn standard_section_full_multiple_paragraphs() {
        let initial_source = "-- p\n\ndelta echo\nwhiskey\n\nmike oscar\npapa\n\n-- hr\n\n";
        let captured_source = "-- p\n\ndelta echo\nwhiskey\n\nmike oscar\npapa".to_string();
        let source = "delta echo\nwhiskey\n\nmike oscar\npapa\n\n-- hr\n\n";
        let r#type = "p";
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let config = Config::mock_basic_config();
        let left = Ok((
            "-- hr\n\n",
            Child::Section(Section {
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                bounds: "full".to_string(),
                category: SectionCategory::StandardSectionFull {
                    containers: vec![
                        Child::Block(vec![
                            Span::Word {
                                text: "delta".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                            Span::Space {
                                text: " ".to_string(),
                                template: "spans/space.jinja".to_string(),
                            },
                            Span::Word {
                                text: "echo".to_string(),
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
                        ]),
                        Child::Block(vec![
                            Span::Word {
                                text: "mike".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                            Span::Space {
                                text: " ".to_string(),
                                template: "spans/space.jinja".to_string(),
                            },
                            Span::Word {
                                text: "oscar".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                            Span::Space {
                                text: "\n".to_string(),
                                template: "spans/space.jinja".to_string(),
                            },
                            Span::Word {
                                text: "papa".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                        ]),
                    ],
                },
                template: "default".to_string(),
                r#type: "p".to_string(),
                source: captured_source,
            }),
        ));
        let right = standard_section_full(
            source,
            r#type,
            key_value_attributes,
            flag_attributes,
            &config,
            initial_source,
        );
        assert_eq!(left, right);
    }
}
