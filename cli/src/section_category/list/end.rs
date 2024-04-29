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
use std::collections::{BTreeMap, BTreeSet};

pub fn list_section_end<'a>(
    source: &'a str,
    r#type: &str,
    key_value_attributes: BTreeMap<String, String>,
    flag_attributes: BTreeSet<String>,
    config: &'a Config,
    initial_source: &str,
) -> IResult<&'a str, Child> {
    if config
        .section_categories
        .list
        .contains(&r#type.to_string())
    {
        let (source, containers) = many0(|src| child(src, config))(source)?;
        let (source, _) = tuple((space0, line_ending))(source)?;
        let (source, _) = multispace0(source)?;
        let section = Child::Section(Section {
            key_value_attributes,
            flag_attributes,
            bounds: "end".to_string(),
            category: SectionCategory::ListSectionEnd { containers },
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
    use pretty_assertions::assert_eq;

    #[test]
    // #[ignore]
    fn standard_section_end_empty_section() {
        let source = "\n\n-- div";
        let r#type = "list";
        let initial_source = "-- /list\n\n-- div";
        let target_section_source_response = "-- /list".to_string();
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let config = Config::set1();
        let left = Ok((
            "-- div",
            Child::Section(Section {
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                bounds: "end".to_string(),
                category: SectionCategory::ListSectionEnd { containers: vec![] },
                template: "default".to_string(),
                r#type: "list".to_string(),
                source: target_section_source_response,
            }),
        ));
        let right = list_section_end(
            source,
            r#type,
            key_value_attributes,
            flag_attributes,
            &config,
            initial_source,
        );
        assert_eq!(left, right);
    }


    // #[test]
    // // #[ignore]
    // fn standard_section_end_one_paragraph() {
    //     let source = "delta lima\n\n-- hr\n\n";
    //     let r#type = "p";
    //     let initial_source = "-- /p\n\ndelta lima\n\n-- hr\n\n";
    //     let target_section_source_response = "-- /p\n\ndelta lima".to_string();
    //     let key_value_attributes = BTreeMap::new();
    //     let flag_attributes = BTreeSet::new();
    //     let config = Config::set1();
    //     let left = Ok((
    //         "-- hr\n\n",
    //         Child::Section(Section {
    //             key_value_attributes: BTreeMap::new(),
    //             flag_attributes: BTreeSet::new(),
    //             bounds: "end".to_string(),
    //             category: SectionCategory::StandardSectionEnd {
    //                 containers: vec![Child::Block(vec![
    //                     Span::WordSegment {
    //                         text: "delta".to_string(),
    //                         template: "spans/word.neojinja".to_string(),
    //                     },
    //                     Span::Space {
    //                         text: " ".to_string(),
    //                         template: "spans/space.neojinja".to_string(),
    //                     },
    //                     Span::WordSegment {
    //                         text: "lima".to_string(),
    //                         template: "spans/word.neojinja".to_string(),
    //                     },
    //                 ])],
    //             },
    //             template: "default".to_string(),
    //             r#type: "p".to_string(),
    //             source: target_section_source_response,
    //         }),
    //     ));
    //     let right = standard_section_end(
    //         source,
    //         r#type,
    //         key_value_attributes,
    //         flag_attributes,
    //         &config,
    //         initial_source,
    //     );
    //     assert_eq!(left, right);
    // }

    // #[test]
    // // #[ignore]
    // fn standard_section_end_single_paragraph_on_multiple_lines() {
    //     let source = "tango alfa\nbravo\n\n-- hr\n\n";
    //     let r#type = "p";
    //     let initial_source = "-- /p\n\ntango alfa\nbravo\n\n-- hr\n\n";
    //     let target_section_source_response = "-- /p\n\ntango alfa\nbravo".to_string();
    //     let key_value_attributes = BTreeMap::new();
    //     let flag_attributes = BTreeSet::new();
    //     let config = Config::set1();
    //     let left = Ok((
    //         "-- hr\n\n",
    //         Child::Section(Section {
    //             key_value_attributes: BTreeMap::new(),
    //             flag_attributes: BTreeSet::new(),
    //             bounds: "end".to_string(),
    //             category: SectionCategory::StandardSectionEnd {
    //                 containers: vec![Child::Block(vec![
    //                     Span::WordSegment {
    //                         text: "tango".to_string(),
    //                         template: "spans/word.neojinja".to_string(),
    //                     },
    //                     Span::Space {
    //                         text: " ".to_string(),
    //                         template: "spans/space.neojinja".to_string(),
    //                     },
    //                     Span::WordSegment {
    //                         text: "alfa".to_string(),
    //                         template: "spans/word.neojinja".to_string(),
    //                     },
    //                     Span::Space {
    //                         text: "\n".to_string(),
    //                         template: "spans/space.neojinja".to_string(),
    //                     },
    //                     Span::WordSegment {
    //                         text: "bravo".to_string(),
    //                         template: "spans/word.neojinja".to_string(),
    //                     },
    //                 ])],
    //             },
    //             template: "default".to_string(),
    //             r#type: "p".to_string(),
    //             source: target_section_source_response,
    //         }),
    //     ));
    //     let right = standard_section_end(
    //         source,
    //         r#type,
    //         key_value_attributes,
    //         flag_attributes,
    //         &config,
    //         initial_source,
    //     );
    //     assert_eq!(left, right);
    // }

    // #[test]
    // // #[ignore]
    // fn standard_section_end_multiple_paragraphs() {
    //     let source = "delta echo\nwhiskey\n\nmike oscar\npapa\n\n-- hr\n\n";
    //     let r#type = "p";
    //     let initial_source = "-- /p\n\ndelta echo\nwhiskey\n\nmike oscar\npapa\n\n-- hr\n\n";
    //     let target_section_source_response =
    //         "-- /p\n\ndelta echo\nwhiskey\n\nmike oscar\npapa".to_string();
    //     let key_value_attributes = BTreeMap::new();
    //     let flag_attributes = BTreeSet::new();
    //     let config = Config::set1();
    //     let left = Ok((
    //         "-- hr\n\n",
    //         Child::Section(Section {
    //             key_value_attributes: BTreeMap::new(),
    //             flag_attributes: BTreeSet::new(),
    //             bounds: "end".to_string(),
    //             category: SectionCategory::StandardSectionEnd {
    //                 containers: vec![
    //                     Child::Block(vec![
    //                         Span::WordSegment {
    //                             text: "delta".to_string(),
    //                             template: "spans/word.neojinja".to_string(),
    //                         },
    //                         Span::Space {
    //                             text: " ".to_string(),
    //                             template: "spans/space.neojinja".to_string(),
    //                         },
    //                         Span::WordSegment {
    //                             text: "echo".to_string(),
    //                             template: "spans/word.neojinja".to_string(),
    //                         },
    //                         Span::Space {
    //                             text: "\n".to_string(),
    //                             template: "spans/space.neojinja".to_string(),
    //                         },
    //                         Span::WordSegment {
    //                             text: "whiskey".to_string(),
    //                             template: "spans/word.neojinja".to_string(),
    //                         },
    //                     ]),
    //                     Child::Block(vec![
    //                         Span::WordSegment {
    //                             text: "mike".to_string(),
    //                             template: "spans/word.neojinja".to_string(),
    //                         },
    //                         Span::Space {
    //                             text: " ".to_string(),
    //                             template: "spans/space.neojinja".to_string(),
    //                         },
    //                         Span::WordSegment {
    //                             text: "oscar".to_string(),
    //                             template: "spans/word.neojinja".to_string(),
    //                         },
    //                         Span::Space {
    //                             text: "\n".to_string(),
    //                             template: "spans/space.neojinja".to_string(),
    //                         },
    //                         Span::WordSegment {
    //                             text: "papa".to_string(),
    //                             template: "spans/word.neojinja".to_string(),
    //                         },
    //                     ]),
    //                 ],
    //             },
    //             template: "default".to_string(),
    //             r#type: "p".to_string(),
    //             source: target_section_source_response,
    //         }),
    //     ));
    //     let right = standard_section_end(
    //         source,
    //         r#type,
    //         key_value_attributes,
    //         flag_attributes,
    //         &config,
    //         initial_source,
    //     );
    //     assert_eq!(left, right);
    // }
}
