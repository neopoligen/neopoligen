use crate::child::Child;
use crate::config::Config;
use crate::section::Section;
use crate::section_category::SectionCategory;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::Err;
use nom::IResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn preformatted_section_start<'a>(
    source: &'a str,
    r#type: &str,
    key_value_attributes: BTreeMap<String, String>,
    flag_attributes: BTreeSet<String>,
    config: &'a Config,
    initial_source: &str,
) -> IResult<&'a str, Child> {
    if config.section_categories.raw.contains(&r#type.to_string()) {
        let end_target = format!("\n-- /{}", r#type);
        let (source, text) = take_until(end_target.as_str())(source)?;
        let (source, _) = tag("\n")(source)?;
        let section = Child::Section(Section {
            key_value_attributes: key_value_attributes.clone(),
            flag_attributes: flag_attributes.clone(),
            bounds: "start".to_string(),
            category: SectionCategory::PreformattedSectionFull {
                text: Some(text.trim().to_string()),
            },
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
    fn preformatted_section_start_integration() {
        let src = "bravo hotel\n\n-- /code";
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let r#type = "code";
        let config = Config::site1_config();
        let initial_source = "-- code\n\nbravo hotel\n\n-- /code";
        let left = Ok((
            "-- /code",
            Child::Section(Section {
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                bounds: "start".to_string(),
                category: SectionCategory::PreformattedSectionFull {
                    text: Some("bravo hotel".to_string()),
                },
                template: "default".to_string(),
                r#type: r#type.to_string(),
                source: "-- code\n\nbravo hotel".to_string(),
            }),
        ));
        let right = preformatted_section_start(
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
