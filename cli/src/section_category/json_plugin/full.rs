use crate::child::Child;
use crate::config::Config;
use crate::json_plugin::JsonPlugin;
use crate::section_attribute::SectionAttribute;
use minijinja::Value;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::Err;
use nom::IResult;
use std::path::PathBuf;
use std::process::Command;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn json_plugin_section_full<'a>(
    source: &'a str,
    r#type: &str,
    attributes: &[SectionAttribute],
    config: &'a Config,
) -> IResult<&'a str, Child> {
    // let types = &config.json_plugins;
    // dbg!(&config.json_plugins);
    if let Some(plugin) = config.json_plugins.get(&r#type.to_string()) {
        let (source, _payload) = take_until("\n--")(source)?;
        let (source, _) = multispace0(source)?;
        let plugin_path = PathBuf::from(format!(
            "/Users/alan/Neopoligen/neopoligen-site/_configuration/plugins/{}/plugin.neop",
            plugin,
        ));
        let params: Vec<String> = attributes
            .iter()
            .filter_map(|attr| {
                if let SectionAttribute::KeyValue { key, value } = attr {
                    if *key == *"params" {
                        Some(value.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        if let Ok(output) = Command::new(plugin_path).args(&params).output() {
            if let Ok(text) = String::from_utf8(output.stdout) {
                match serde_json::from_str::<Value>(text.as_str()) {
                    Ok(data) => Ok((
                        source,
                        Child::JsonPlugin(JsonPlugin {
                            key_value_attributes: BTreeMap::new(),
                            flag_attributes: BTreeSet::new(),
                            bounds: "full".to_string(),
                            template: "default".to_string(),
                            object: Some(data),
                            r#type: r#type.to_string(),
                        }),
                    )),
                    Err(_) => Err(Err::Error(Error::new(source, ErrorKind::Tag))),
                }
            } else {
                // TODO: Figure out how to pass the actual error kind
                // hear instead of hard coding to TakeUntil
                Err(Err::Error(Error::new(source, ErrorKind::Not)))
            }
        } else {
            // TODO: Figure out how to pass the actual error kind
            // hear instead of hard coding to TakeUntil
            Err(Err::Error(Error::new(source, ErrorKind::Not)))
        }
    } else {
        // TODO: Figure out how to pass the actual error kind
        // hear instead of hard coding to TakeUntil
        Err(Err::Error(Error::new(source, ErrorKind::TakeUntil)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::child::Child;
    use crate::config::Config;
    use pretty_assertions::assert_eq;

    #[test]
    #[ignore]
    fn json_plugin_section_full_no_params() {
        let source = "\n\n-- hr";
        let config = Config::site1_config();
        let r#type = "random-color-circle";
        let attributes = vec![];
        let left = Ok((
            "-- hr",
            Child::JsonPlugin(JsonPlugin {
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                bounds: "full".to_string(),
                template: "default".to_string(),
                object: Some(serde_json::from_str::<Value>("{ \"color\": \"#bbrrtt\" }").unwrap()),
                r#type: "random-color-circle".to_string(),
            }),
        ));
        let right = json_plugin_section_full(source, r#type, &attributes, &config);
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn json_plugin_section_full_with_params() {
        let source = "\n\n-- hr";
        let config = Config::site1_config();
        let r#type = "random-color-circle";
        let attributes = vec![
            SectionAttribute::KeyValue {
                key: "params".to_string(),
                value: "--base".to_string(),
            },
            SectionAttribute::KeyValue {
                key: "params".to_string(),
                value: "green".to_string(),
            },
        ];
        let left = Ok((
            "-- hr",
            Child::JsonPlugin(JsonPlugin {

                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                // attributes: vec![
                //     SectionAttribute::KeyValue {
                //         key: "params".to_string(),
                //         value: "--base".to_string(),
                //     },
                //     SectionAttribute::KeyValue {
                //         key: "params".to_string(),
                //         value: "green".to_string(),
                //     },
                // ],
                bounds: "full".to_string(),
                template: "default".to_string(),
                object: Some(serde_json::from_str::<Value>("{ \"color\": \"#aaiill\" }").unwrap()),
                r#type: "random-color-circle".to_string(),
            }),
        ));
        let right = json_plugin_section_full(source, r#type, &attributes, &config);
        assert_eq!(left, right);
    }
}
