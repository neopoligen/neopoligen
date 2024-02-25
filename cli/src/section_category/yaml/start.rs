use crate::child::Child;
use crate::config::Config;
use crate::section::Section;
use crate::section_category::SectionCategory;
use minijinja::Value;
use nom::bytes::complete::take_until;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::Err;
use nom::IResult;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use nom::bytes::complete::tag;

pub fn yaml_section_start<'a>(
    source: &'a str,
    r#type: &str,
    key_value_attributes: BTreeMap<String, String>,
    flag_attributes: BTreeSet<String>,
    config: &'a Config,
    initial_source: &str,
) -> IResult<&'a str, Child> {
    // let types = config.section_categories.get("yaml").unwrap();
    // if types.contains(&r#type.to_string()) {
    if config.section_categories.yaml.contains(&r#type.to_string()) {
        let end_target = format!("\n-- /{}", r#type);
        let (source, text) = take_until(end_target.as_str())(source)?;
        let (source, _) = tag("\n")(source)?;
        let de = serde_yaml::Deserializer::from_str(text);
        let object = match Value::deserialize(de) {
            Ok(data) => {
                if data == Value::from(()) {
                    None
                } else {
                    Some(data)
                }
            }
            Err(_e) => None,
        };
        let section = Child::Section(Section {
            key_value_attributes,
            flag_attributes,
            bounds: "start".to_string(),
            category: SectionCategory::YamlSectionStart { object },
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
        Err(Err::Error(Error::new(source, ErrorKind::Not)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn yaml_section_full_integration() {
        let src = r#"
- Casablanca
- North by Northwest
- The Man Who Wasn't There

-- /yaml-example"#;
        let r#type = "yaml-example";
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let config = Config::site1_config();
        let initial_source = r#"-- yaml-example 

- Casablanca
- North by Northwest
- The Man Who Wasn't There"#; 
        let de = serde_yaml::Deserializer::from_str(
            r#"
- Casablanca
- North by Northwest
- The Man Who Wasn't There
            "#,
        );
        let target_yaml = match Value::deserialize(de) {
            Ok(data) => Some(data),
            Err(_e) => None,
        };
        let left = Ok((
            "-- /yaml-example",
            Child::Section(Section {
                key_value_attributes: key_value_attributes.clone(),
                flag_attributes: flag_attributes.clone(),
                bounds: "start".to_string(),
                category: SectionCategory::YamlSectionStart {
                    object: target_yaml,
                },
                template: "default".to_string(),
                r#type: r#type.to_string(),
                source: initial_source.to_string(),
            }),
        ));
        let right = yaml_section_start(
            src,
            r#type,
            key_value_attributes.clone(),
            flag_attributes.clone(),
            &config,
            initial_source,
        );
        assert_eq!(left, right);
    }

    #[test]
    //    #[ignore]
    fn yaml_section_full_section_with_no_data() {
        let src = r#"


-- /yaml-example"#;
        let r#type = "yaml-example";
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let config = Config::site1_config();
        let initial_source = "-- yaml-example";
        let left = Ok((
            "-- /yaml-example",
            Child::Section(Section {
                key_value_attributes: key_value_attributes.clone(),
                flag_attributes: flag_attributes.clone(),
                bounds: "start".to_string(),
                category: SectionCategory::YamlSectionStart { object: None },
                template: "default".to_string(),
                r#type: r#type.to_string(),
                source: "-- yaml-example".to_string(),
            }),
        ));
        let right = yaml_section_start(
            src,
            r#type,
            key_value_attributes.clone(),
            flag_attributes.clone(),
            &config,
            initial_source,
        );
        assert_eq!(left, right);
    }
}
