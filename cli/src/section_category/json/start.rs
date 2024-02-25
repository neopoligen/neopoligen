use crate::child::Child;
use crate::config::Config;
use crate::section::Section;
use crate::section_category::SectionCategory;
use nom::bytes::complete::take_until;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::Err;
use nom::IResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use nom::bytes::complete::tag;

pub fn json_section_start<'a>(
    source: &'a str,
    r#type: &str,
    key_value_attributes: BTreeMap<String, String>,
    flag_attributes: BTreeSet<String>,
    config: &'a Config,
    initial_source: &str,
) -> IResult<&'a str, Child> {
    if config.section_categories.json.contains(&r#type.to_string()) {
        let end_target = format!("\n-- /{}", r#type);
        let (source, text) = take_until(end_target.as_str())(source)?;
        let (source, _) = tag("\n")(source)?;
        let object = match serde_json::from_str(text) {
            Ok(o) => Some(o),
            Err(_e) => None,
        };
        let section = Child::Section(Section {
            key_value_attributes,
            flag_attributes,
            bounds: "start".to_string(),
            category: SectionCategory::JsonSectionStart { object },
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
    fn json_section_full_integration() {
        let src = r#"
        
{"foxtrot": "alfa" }

-- /metadata"#;
        let r#type = "metadata";
        let key_value_attributes = BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let config = Config::mock_basic_config();
        let target_json = serde_json::from_str(r#"{"foxtrot": "alfa" }"#).unwrap();
        let initial_source = r#"-- metadata/

-- this is not valid json
{"foxtrot": "alfa" }"#;
        let left = Ok((
            "-- /metadata",
            Child::Section(Section {
                key_value_attributes: key_value_attributes.clone(),
                flag_attributes: flag_attributes.clone(),
                bounds: "start".to_string(),
                category: SectionCategory::JsonSectionStart {
                    object: Some(target_json),
                },
                template: "default".to_string(),
                r#type: r#type.to_string(),
                source: r#"-- metadata/

-- this is not valid json
{"foxtrot": "alfa" }"#.to_string(),
            }),
        ));
        let right = json_section_start(
            src,
            r#type,
            key_value_attributes.clone(),
            flag_attributes.clone(),
            &config,
            initial_source,
        );
        assert_eq!(left, right);
    }

//     #[test]
//     fn json_section_full_section_with_no_data() {
//         let src = r#"
        
// -- p"#;
//         let r#type = "metadata";
//         let key_value_attributes = BTreeMap::new();
//         let flag_attributes = BTreeSet::new();
//         let config = Config::mock_basic_config();
//         let initial_source = r#"-- metadata"#;
//         let left = Ok((
//             "-- p",
//             Child::Section(Section {
//                 key_value_attributes: key_value_attributes.clone(),
//                 flag_attributes: flag_attributes.clone(),
//                 bounds: "full".to_string(),
//                 category: SectionCategory::JsonSectionFull { object: None },
//                 template: "default".to_string(),
//                 r#type: r#type.to_string(),
//                 source: "-- metadata".to_string(),
//             }),
//         ));
//         let right = json_section_full(
//             src,
//             r#type,
//             key_value_attributes.clone(),
//             flag_attributes.clone(),
//             &config,
//             initial_source,);
//         assert_eq!(left, right);
//     }
}
