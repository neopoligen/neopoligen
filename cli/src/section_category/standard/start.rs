use crate::child::child;
use crate::child::Child;
use crate::config::Config;
use crate::section::Section;
use crate::section_category::SectionCategory;
use nom::character::complete::multispace0;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::Err;
use nom::IResult;
use std::collections::{BTreeMap, BTreeSet};
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::sequence::tuple;

pub fn standard_section_start<'a>(
    source: &'a str,
    r#type: &str,
    key_value_attributes: BTreeMap<String, String>,
    flag_attributes: BTreeSet<String>,
    config: &'a Config,
    initial_source: &str,
) -> IResult<&'a str, Child> {
    if config.section_categories.standard.contains(&r#type.to_string()) {
        let (source, containers) = many0(|src| child(src, config))(source)?;
        let (source, _) = tuple((space0, line_ending))(source)?;
        let (source, _) = multispace0(source)?;
        let section = Child::Section(Section {
            key_value_attributes,
            flag_attributes,
            bounds: "start".to_string(),
            category: SectionCategory::StandardSectionStart { containers },
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
    fn standard_section_start_empty_section() {
        let src = "\n\n-- /tldr\n\n-- div";
        let initial_source = "-- tldr/\n\n-- /tldr\n\n-- div";
        let target_section_source_response = "-- tldr/".to_string();
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let r#type = "tldr";
        let config = Config::mock_basic_config();
        let left = Ok((
            "-- /tldr\n\n-- div",
            Child::Section(Section {
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                bounds: "start".to_string(),
                category: SectionCategory::StandardSectionStart { containers: vec![] },
                template: "default".to_string(),
                r#type: "tldr".to_string(),
                source: target_section_source_response,
            }),
        ));
        let right = standard_section_start(
            src,
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
    fn standard_section_start_one_paragraph() {
        let src = "delta lima\n\n-- /div\n\n-- hr\n\n";
        let initial_source = "-- div/\n\ndelta lima\n\n-- /div\n\n-- hr\n\n";
        let target_section_source_response = "-- div/\n\ndelta lima".to_string();
        let r#type = "div";
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let config = Config::mock_basic_config();
        let left = Ok((
            "-- /div\n\n-- hr\n\n",
            Child::Section(Section {
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                bounds: "start".to_string(),
                category: SectionCategory::StandardSectionStart {
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
                r#type: "div".to_string(),
                source: target_section_source_response,
            }),
        ));
        let right = standard_section_start(
            src,
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
    fn standard_section_start_single_paragraph_on_multiple_lines() {
        let src = "tango alfa\nbravo\n\n-- /p\n\n-- hr\n\n";
        let initial_source = "-- p/\n\ntango alfa\nbravo\n\n-- /p\n\n-- hr\n\n";
        let target_section_source_response = "-- p/\n\ntango alfa\nbravo".to_string();
        let r#type = "p";
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let config = Config::mock_basic_config();
        let left = Ok((
            "-- /p\n\n-- hr\n\n",
            Child::Section(Section {
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                bounds: "start".to_string(),
                category: SectionCategory::StandardSectionStart {
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
                source: target_section_source_response,
            }),
        ));
        let right = standard_section_start(
            src,
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
    fn standard_section_start_multiple_paragraphs() {
        let src = "delta echo\nwhiskey\n\nmike oscar\npapa\n\n-- /div\n\n-- hr\n\n";
        let initial_source = "-- div/\n\ndelta echo\nwhiskey\n\nmike oscar\npapa\n\n-- /div\n\n-- hr\n\n";
        let target_section_source_response = "-- div/\n\ndelta echo\nwhiskey\n\nmike oscar\npapa".to_string();
        let r#type = "div";
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let config = Config::mock_basic_config();
        let left = Ok((
            "-- /div\n\n-- hr\n\n",
            Child::Section(Section {
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                bounds: "start".to_string(),
                category: SectionCategory::StandardSectionStart {
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
                r#type: "div".to_string(),
                source: target_section_source_response,
            }),
        ));
        let right = standard_section_start(
            src,
            r#type,
            key_value_attributes,
            flag_attributes,
            &config,
            initial_source,
        );
        assert_eq!(left, right);
    }
}
