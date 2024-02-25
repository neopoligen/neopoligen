use crate::child::*;
use crate::config::Config;
use nom::multi::many1;
use nom::IResult;

pub fn ast<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Vec<Child>> {
    let (unparsed, ast) = many1(|src| child(src, config))(source)?;
    Ok((unparsed, ast))
}

#[cfg(test)]
mod test {

    // use super::*;
    // use crate::section::Section;
    // use crate::section_category::SectionCategory;
    // use crate::span::Span;
    // use pretty_assertions::assert_eq;
    // use std::collections::BTreeMap;
    // use std::collections::BTreeSet;

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
