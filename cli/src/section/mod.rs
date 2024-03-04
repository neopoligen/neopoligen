use crate::child::Child;
use crate::config::Config;
use crate::section_attribute::section_attribute;
use crate::section_attribute::SectionAttribute;
use crate::section_category::comment::end::*;
use crate::section_category::comment::full::*;
use crate::section_category::comment::start::*;
use crate::section_category::json::end::*;
use crate::section_category::json::full::*;
use crate::section_category::json::start::*;
use crate::section_category::json_plugin::full::*;
use crate::section_category::raw::end::*;
use crate::section_category::raw::full::*;
use crate::section_category::raw::start::*;
use crate::section_category::standard::end::*;
use crate::section_category::standard::full::*;
use crate::section_category::standard::start::*;
use crate::section_category::text_plugin::full::*;
use crate::section_category::yaml::end::*;
use crate::section_category::yaml::full::*;
use crate::section_category::yaml::start::*;
use crate::section_category::SectionCategory;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Section {
    pub key_value_attributes: BTreeMap<String, String>,
    pub flag_attributes: BTreeSet<String>,
    pub bounds: String,
    pub category: SectionCategory,
    pub template: String,
    pub r#type: String,
    pub source: String,
}

pub fn section<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Child> {
    let (source, sec) = alt((
        |src| section_full(src, config),
        |src| section_start(src, config),
        |src| section_end(src, config),
    ))(source)?;
    Ok((source, sec))
}

pub fn section_full<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Child> {
    let initial_source = source;
    let (source, r#type) = preceded(tag_no_case("-- "), is_not(" \n\t"))(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    // attributes key/value and flags
    let (source, attributes) = many0(section_attribute)(source)?;
    let mut key_value_attributes: BTreeMap<String, String> = BTreeMap::new();
    let _ = &attributes
        .clone()
        .into_iter()
        .filter_map(|a| match a {
            SectionAttribute::KeyValue { key, value } => Some((key, value)),
            _ => None,
        })
        .for_each(|a| {
            match key_value_attributes.get(&a.0) {
                Some(initial_value) => key_value_attributes
                    .insert(a.0.to_string(), format!("{} {}", initial_value, &a.1)),
                None => key_value_attributes.insert(a.0, a.1),
            };
        });
    let mut flag_attributes: BTreeSet<String> = BTreeSet::new();
    let _ = &attributes
        .clone()
        .into_iter()
        .filter_map(|a| match a {
            SectionAttribute::Bool { key } => Some(key),
            _ => None,
        })
        .for_each(|a| {
            flag_attributes.insert(a.to_string());
        });
    // end attributes

    let (source, sec) = alt((
        // TODO: Remove all &attributes in favor of &key_value_attributes and &flag_attributes
        |src| {
            comment_section_full(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| {
            json_section_full(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| {
            standard_section_full(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| {
            preformatted_section_full(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| {
            yaml_section_full(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| text_plugin_section_full(src, r#type, &attributes, config),
        |src| json_plugin_section_full(src, r#type, &attributes, config),
    ))(source)?;
    Ok((source, sec))
}

pub fn section_start<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Child> {
    let initial_source = source;
    let (source, r#type) = preceded(tag_no_case("-- "), is_not("/ \n\t"))(source)?;
    let (source, _) = tag("/")(source)?;
    let (source, _) = tag("\n")(source)?;
    // start attributes key/value and flags
    let (source, attributes) = many0(section_attribute)(source)?;
    let mut key_value_attributes: BTreeMap<String, String> = BTreeMap::new();
    let _ = &attributes
        .clone()
        .into_iter()
        .filter_map(|a| match a {
            SectionAttribute::KeyValue { key, value } => Some((key, value)),
            _ => None,
        })
        .for_each(|a| {
            match key_value_attributes.get(&a.0) {
                Some(initial_value) => key_value_attributes
                    .insert(a.0.to_string(), format!("{} {}", initial_value, &a.1)),
                None => key_value_attributes.insert(a.0, a.1),
            };
        });
    let mut flag_attributes: BTreeSet<String> = BTreeSet::new();
    let _ = &attributes
        .clone()
        .into_iter()
        .filter_map(|a| match a {
            SectionAttribute::Bool { key } => Some(key),
            _ => None,
        })
        .for_each(|a| {
            flag_attributes.insert(a.to_string());
        });
    // end attributes
    let (source, sec) = alt((
        |src| {
            comment_section_start(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| {
            json_section_start(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| {
            preformatted_section_start(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| {
            standard_section_start(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| {
            yaml_section_start(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
    ))(source)?;
    Ok((source, sec))
}

pub fn section_end<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Child> {
    let initial_source = source;
    let (source, r#type) = preceded(tag_no_case("-- /"), is_not(" \n\t"))(source)?;
    let (source, _) = not_line_ending(source)?;
    let (source, _) = line_ending(source)?;
    // start attributes key/value and flags
    let (source, attributes) = many0(section_attribute)(source)?;
    let mut key_value_attributes: BTreeMap<String, String> = BTreeMap::new();
    let _ = &attributes
        .clone()
        .into_iter()
        .filter_map(|a| match a {
            SectionAttribute::KeyValue { key, value } => Some((key, value)),
            _ => None,
        })
        .for_each(|a| {
            match key_value_attributes.get(&a.0) {
                Some(initial_value) => key_value_attributes
                    .insert(a.0.to_string(), format!("{} {}", initial_value, &a.1)),
                None => key_value_attributes.insert(a.0, a.1),
            };
        });
    let mut flag_attributes: BTreeSet<String> = BTreeSet::new();
    let _ = &attributes
        .clone()
        .into_iter()
        .filter_map(|a| match a {
            SectionAttribute::Bool { key } => Some(key),
            _ => None,
        })
        .for_each(|a| {
            flag_attributes.insert(a.to_string());
        });
    // end attributes
    let (source, sec) = alt((
        |src| {
            comment_section_end(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| {
            json_section_end(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| {
            preformatted_section_end(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| {
            standard_section_end(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| {
            yaml_section_end(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
    ))(source)?;
    Ok((source, sec))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;

    #[test]
    // #[ignore]
    fn section_integration_full() {
        let source = "-- title\n-- class: alfa\n-- class: bravo\n\nTitle Echo\n\n-- p";
        let config = Config::set1();
        let mut key_value_attributes = BTreeMap::new();
        key_value_attributes.insert("class".to_string(), "alfa bravo".to_string());
        let left = Ok((
            "-- p",
            Child::Section(Section {
                key_value_attributes,
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
                            text: "Echo".to_string(),
                            template: "spans/word.jinja".to_string(),
                        },
                    ])],
                },
                template: "default".to_string(),
                r#type: "title".to_string(),
                source: "-- title\n-- class: alfa\n-- class: bravo\n\nTitle Echo".to_string(),
            }),
        ));
        let right = section(source, &config);
        assert_eq!(left, right);
    }

    #[test]
    // #[ignore]
    fn allow_spaces_after_tags() {
        // this is exactly the same as above but there's a space after the
        // `-- title`` to make sure that doesn't cause issues
        let source = "-- title \n-- class: alfa\n-- class: bravo\n\nTitle Echo\n\n-- p";
        let config = Config::set1();
        let mut key_value_attributes = BTreeMap::new();
        key_value_attributes.insert("class".to_string(), "alfa bravo".to_string());
        let left = Ok((
            "-- p",
            Child::Section(Section {
                key_value_attributes,
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
                            text: "Echo".to_string(),
                            template: "spans/word.jinja".to_string(),
                        },
                    ])],
                },
                template: "default".to_string(),
                r#type: "title".to_string(),
                source: "-- title \n-- class: alfa\n-- class: bravo\n\nTitle Echo".to_string(),
            }),
        ));
        let right = section(source, &config);
        assert_eq!(left, right);
    }
}
