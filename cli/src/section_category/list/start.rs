use crate::child::Child;
use crate::config::Config;
use crate::list::Item;
use crate::list::List;
use crate::section::section;
use crate::span::span;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::Err;
use nom::IResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn list_section_start<'a>(
    source: &'a str,
    r#type: &str,
    key_value_attributes: BTreeMap<String, String>,
    flag_attributes: BTreeSet<String>,
    config: &'a Config,
    initial_source: &str,
) -> IResult<&'a str, Child> {
    if config.section_categories.list.contains(&r#type.to_string()) {
        let (source, _) = multispace0(source)?;
        let (source, items) = many0(|src| list_item_wrapper(src, config))(source)?;
        // let (source, _) = tuple((space0, line_ending))(source)?;
        // let (source, _) = multispace0(source)?;
        let list = Child::List(List {
            key_value_attributes,
            flag_attributes,
            r#type: r#type.to_string(),
            bounds: "start".to_string(),
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
        // hear instead of hard coding to TakeUntil,
        // maybe look at match instead of an if
        Err(Err::Error(Error::new(source, ErrorKind::TakeUntil)))
    }
}

pub fn list_item_wrapper<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Item> {
    let (source, _) = not(tag("--"))(source)?;
    let (source, _) = tag("-")(source)?;
    let (source, _) = space1(source)?;
    let (source, items) = many0(alt((
        |src| list_item_block(src, config),
        |src| section(src, config),
    )))(source)?;
    let (source, _) = multispace0(source)?;
    Ok((source, Item::ListItem { containers: items }))
}

pub fn list_item_block<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Child> {
    let (source, _) = multispace0(source)?;
    let (source, _) = not(tag("-"))(source)?;
    let (source, response) = many0(|src| span(src, config))(source)?;
    let (source, _) = tag("\n")(source)?;
    Ok((source, Child::Block(response)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;

    #[test]
    // #[ignore]
    fn list_section_integration() {
        let src = "- whiskey juliette\n\n- romeo\nvictor\n\n-- p\n\n";
        let initial_source = "-- notes\n\n- whiskey juliette\n\n- romeo\nvictor\n\n-- p\n\n";
        let r#type = "notes";
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let config = Config::site1_config();
        let left = Ok((
            "-- p\n\n",
            Child::List(List {
                key_value_attributes: key_value_attributes.clone(),
                flag_attributes: flag_attributes.clone(),
                source: "-- notes\n\n- whiskey juliette\n\n- romeo\nvictor".to_string(),
                r#type: "notes".to_string(),
                bounds: "start".to_string(),
                template: "default".to_string(),
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
                                text: "juliette".to_string(),
                                template: "spans/word.jinja".to_string(),
                            },
                        ])],
                    },
                    Item::ListItem {
                        containers: vec![Child::Block(vec![
                            Span::Word {
                                text: "romeo".to_string(),
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
        ));
        let right = list_section_start(
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
