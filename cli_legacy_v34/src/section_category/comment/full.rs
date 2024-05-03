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

pub fn comment_section_full<'a>(
    source: &'a str,
    r#type: &str,
    key_value_attributes: BTreeMap<String, String>,
    flag_attributes: BTreeSet<String>,
    config: &'a Config,
    initial_source: &str,
) -> IResult<&'a str, Child> {
    // let types = config.section_categories.get("hidden").unwrap();
    // if types.contains(&r#type.to_string()) {
    if config.section_categories.comment.contains(&r#type.to_string()) {
        let (source, containers) = many0(|src| child(src, config))(source)?;
        let (source, _) = tuple((space0, line_ending))(source)?;
        let (source, _) = multispace0(source)?;
        let section = Child::Section(Section {
            key_value_attributes,
            flag_attributes,
            bounds: "full".to_string(),
            category: SectionCategory::CommentSectionFull { containers },
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
        // TODO: Figure out how to pass the actual error kind
        // hear instead of hard coding to TakeUntil
        Err(Err::Error(Error::new(source, ErrorKind::TakeUntil)))
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    // use pretty_assertions::assert_eq;

    // #[test]
    // // #[ignore]
    // fn comment_section_full_empty() {
    //     let source = "\n\n-- p\n\n";
    //     let r#type = "comment";
    //     let attributes = vec![];
    //     let config = Config::set1();
    //     let left = Ok((
    //         "-- p\n\n",
    //         Child::Section(Section {
    //             key_value_attributes: BTreeMap::new(),
    //             flag_attributes: BTreeSet::new(),
    //             bounds: "full".to_string(),
    //             category: SectionCategory::CommentSectionFull,
    //             template: "default".to_string(),
    //             r#type: "comment".to_string(),
    //             source: "".to_string(),
    //         }),
    //     ));
    //     let right = comment_section_full(source, r#type, &attributes, &config);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // // #[ignore]
    // fn comment_section_full_with_text() {
    //     let source = "nothing to see here\n\n-- hr\n\n";
    //     let r#type = "comment";
    //     let attributes = vec![];
    //     let config = Config::set1();
    //     let left = Ok((
    //         "-- hr\n\n",
    //         Child::Section(Section {
    //             key_value_attributes: BTreeMap::new(),
    //             flag_attributes: BTreeSet::new(),
    //             bounds: "full".to_string(),
    //             category: SectionCategory::CommentSectionFull,
    //             template: "default".to_string(),
    //             r#type: "comment".to_string(),
    //             source: "".to_string(),
    //         }),
    //     ));
    //     let right = comment_section_full(source, r#type, &attributes, &config);
    //     assert_eq!(left, right);
    // }
}
