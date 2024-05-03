use crate::child::Child;
use crate::config::Config;
use crate::section_attribute::section_attribute;
use crate::section_attribute::SectionAttribute;
use crate::section_category::checklist::end::checklist_section_end;
use crate::section_category::checklist::full::checklist_section_full;
use crate::section_category::checklist::start::checklist_section_start;
use crate::section_category::list::end::list_section_end;
use crate::section_category::list::full::list_section_full;
use crate::section_category::list::start::list_section_start;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct List {
    pub key_value_attributes: BTreeMap<String, String>,
    pub flag_attributes: BTreeSet<String>,
    pub template: String,
    pub r#type: String,
    pub bounds: String,
    pub items: Vec<Item>,
    pub source: String,
    // TODO: Add attributes
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(content = "content", rename_all = "lowercase", tag = "type")]
pub enum Item {
    ListItem {
        containers: Vec<Child>,
    },
    ChecklistItem {
        status: Option<String>,
        containers: Vec<Child>,
    },
}

pub fn list<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Child> {
    let (source, sec) = alt((
        |src| list_full(src, config),
        |src| list_start(src, config),
        |src| list_end(src, config),
    ))(source)?;
    Ok((source, sec))
}

pub fn list_full<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Child> {
    let initial_source = source;
    let (source, r#type) = preceded(tag_no_case("-- "), is_not(" \n\t"))(source)?;
    let (source, _) = line_ending(source)?;
    let (source, attributes) = many0(section_attribute)(source)?;
    let (source, _) = multispace0(source)?;
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

    let (source, section) = alt((
        |src| {
            checklist_section_full(
                src,
                r#type,
                &key_value_attributes,
                &flag_attributes,
                config,
                initial_source,
            )
        },
        |src| {
            list_section_full(
                src,
                r#type,
                &key_value_attributes,
                &flag_attributes,
                config,
                initial_source,
            )
        },
    ))(source)?;
    Ok((source, section))
}

pub fn list_start<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Child> {
    let initial_source = source;
    let (source, r#type) = preceded(tag_no_case("-- "), is_not(" /\n\t"))(source)?;
    let (source, _) = tag("/")(source)?;
    let (source, _) = line_ending(source)?;
    let (source, attributes) = many0(section_attribute)(source)?;
    let (source, _) = multispace0(source)?;
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

    let (source, section) = alt((
        |src| {
            checklist_section_start(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| {
            list_section_start(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
    ))(source)?;
    Ok((source, section))
}

pub fn list_end<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Child> {
    let initial_source = source;
    let (source, r#type) = preceded(tag_no_case("-- /"), is_not(" \n\t"))(source)?;
    let (source, _) = not_line_ending(source)?;
    let (source, _) = line_ending(source)?;
    let (source, attributes) = many0(section_attribute)(source)?;
    // let (source, _) = multispace0(source)?;
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

    let (source, section) = alt((
        |src| {
            checklist_section_end(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
        |src| {
            list_section_end(
                src,
                r#type,
                key_value_attributes.clone(),
                flag_attributes.clone(),
                config,
                initial_source,
            )
        },
    ))(source)?;
    Ok((source, section))
}

#[cfg(test)]
mod test {
    // TODO: Add some tests here to identify start/full/end
    // properly
}
