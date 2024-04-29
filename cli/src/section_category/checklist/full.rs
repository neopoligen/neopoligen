use crate::child::Child;
use crate::config::Config;
use crate::list::Item;
use crate::list::List;
use crate::section::section;
// use crate::section_attribute::SectionAttribute;
use crate::span::span;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::not;
use nom::combinator::opt;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::Err;
use nom::IResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn checklist_section_full<'a>(
    source: &'a str,
    r#type: &str,
    key_value_attributes: &BTreeMap<String, String>,
    flag_attributes: &BTreeSet<String>,
    config: &'a Config,
    initial_source: &str,
) -> IResult<&'a str, Child> {
    // let types = config.checklist_sections.get("checklist_sections").unwrap();
    if config
        .section_categories
        .checklist
        .contains(&r#type.to_string())
    {
        let (source, _) = multispace0(source)?;
        let (source, items) = many0(|src| checklist_item_wrapper(src, config))(source)?;
        let list = Child::List(List {
            key_value_attributes: key_value_attributes.clone(),
            flag_attributes: flag_attributes.clone(),
            r#type: r#type.to_string(),
            bounds: "full".to_string(),
            template: "default".to_string(),
            items,
            source: initial_source
                .replace(source, "")
                .as_str()
                .trim()
                .to_string(),
        });
        Ok((source, list))
    } else {
        // TODO: Figure out how to pass the actual error kind
        // hear instead of hard coding to TakeUntil
        Err(Err::Error(Error::new(source, ErrorKind::TakeUntil)))
    }
}

pub fn checklist_item_wrapper<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Item> {
    let (source, _) = not(tag("--"))(source)?;
    let (source, _) = tag("[")(source)?;
    let (source, tmp_status) = opt(is_not("]"))(source)?;
    let (source, _) = tag("]")(source)?;
    let status = tmp_status.map(|x| x.to_string());
    let (source, items) = many0(alt((
        |src| checklist_item_block(src, config),
        |src| section(src, config),
    )))(source)?;
    let (source, _) = multispace0(source)?;
    Ok((
        source,
        Item::ChecklistItem {
            status,
            containers: items,
        },
    ))
}

pub fn checklist_item_block<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Child> {
    let (source, _) = multispace0(source)?;
    let (source, _) = not(tag("["))(source)?;
    let (source, _) = not(tag("--"))(source)?;
    let (source, response) = many0(|src| span(src, config))(source)?;
    let (source, _) = tag("\n")(source)?;
    Ok((source, Child::Block(response)))
}

#[cfg(test)]
mod test {
    // use super::*;
    // use crate::span::Span;
    // use pretty_assertions::assert_eq;

    // #[test]
    // // #[ignore]
    // fn checklist_section_integration() {
    //     let source = "[x] delta hotel\n\n[] sierra\necho\n\n-- p\n\n";
    //     let r#type = "todo";
    //     let attributes: Vec<SectionAttribute> = vec![];
    //     let config = Config::set1();
    //     let left = Ok((
    //         "-- p\n\n",
    //         Child::List(List {
    //             r#type: "todo".to_string(),
    //             bounds: "full".to_string(),
    //             template: "default".to_string(),
    //             items: vec![
    //                 Item::ChecklistItem {
    //                     status: Some("x".to_string()),
    //                     containers: vec![Child::Block(vec![
    //                         Span::WordSegment {
    //                             text: "delta".to_string(),
    //                             template: "spans/word.neojinja".to_string(),
    //                         },
    //                         Span::Space {
    //                             text: " ".to_string(),
    //                             template: "spans/space.neojinja".to_string(),
    //                         },
    //                         Span::WordSegment {
    //                             text: "hotel".to_string(),
    //                             template: "spans/word.neojinja".to_string(),
    //                         },
    //                     ])],
    //                 },
    //                 Item::ChecklistItem {
    //                     status: None,
    //                     containers: vec![Child::Block(vec![
    //                         Span::WordSegment {
    //                             text: "sierra".to_string(),
    //                             template: "spans/word.neojinja".to_string(),
    //                         },
    //                         Span::Space {
    //                             text: "\n".to_string(),
    //                             template: "spans/space.neojinja".to_string(),
    //                         },
    //                         Span::WordSegment {
    //                             text: "echo".to_string(),
    //                             template: "spans/word.neojinja".to_string(),
    //                         },
    //                     ])],
    //                 },
    //             ],
    //         }),
    //     ));
    //     let right = checklist_section_full(source, r#type, &attributes, &config);
    //     assert_eq!(left, right);
    // }
}
