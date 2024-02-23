use crate::child::Child;
use crate::config::Config;
use crate::section::Section;
use crate::section_category::SectionCategory;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::Err;
use nom::IResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn preformatted_section_full<'a>(
    source: &'a str,
    r#type: &str,
    key_value_attributes: BTreeMap<String, String>,
    flag_attributes: BTreeSet<String>,
    config: &'a Config,
    initial_source: &str,
) -> IResult<&'a str, Child> {
    if config
        .section_categories
        .preformatted
        .contains(&r#type.to_string())
    {
        let (source, text) = take_until("\n--")(source)?;
        let (source, _) = multispace0(source)?;
        let section = Child::Section(Section {
            key_value_attributes: key_value_attributes.clone(),
            flag_attributes: flag_attributes.clone(),
            bounds: "full".to_string(),
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
        // TODO: Figure out how to pass the actual error kind
        // hear instead of hard coding to TakeUntil
        Err(Err::Error(Error::new(source, ErrorKind::TakeUntil)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn preformatted_section_full_integration() {
        let source = "hotel whiskey\n\n-- p\n\n";
        let r#type = "code";
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let config = Config::mock_basic_config();
        let initial_source = "-- pre\n\nhotel whiskey\n\n-- p\n\n";
        let left = Ok((
            "-- p\n\n",
            Child::Section(Section {
                key_value_attributes: key_value_attributes.clone(),
                flag_attributes: flag_attributes.clone(),
                bounds: "full".to_string(),
                category: SectionCategory::PreformattedSectionFull {
                    text: Some("hotel whiskey".to_string()),
                },
                template: "default".to_string(),
                r#type: r#type.to_string(),
                source: "-- pre\n\nhotel whiskey".to_string(),
            }),
        ));
        let right = preformatted_section_full(
            source,
            r#type,
            key_value_attributes.clone(),
            flag_attributes.clone(),
            &config,
            initial_source,
        );
        assert_eq!(left, right);
    }
}
