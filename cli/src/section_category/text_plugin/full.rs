use crate::child::Child;
use crate::config::Config;
use crate::section_attribute::SectionAttribute;
use crate::text_plugin::TextPlugin;
use nom::character::complete::multispace0;
use nom::bytes::complete::take_until;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::Err;
use nom::IResult;
use std::path::PathBuf;
use std::process::Command;

pub fn text_plugin_section_full<'a>(
    source: &'a str,
    r#type: &str,
    attributes: &[SectionAttribute],
    config: &'a Config,
) -> IResult<&'a str, Child> {
    // let types = &config.text_plugins;
    if let Some(plugin) = config.text_plugins.get(&r#type.to_string()) {
        let (source, _payload) = take_until("\n--")(source)?;
        let (source, _) = multispace0(source)?;
        let plugin_path = PathBuf::from(format!(
            "/Users/alan/Neopoligen/neopoligen-site/configuration/plugins/{}/plugin.neop",
            // &config.dirs.plugins.display(),
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
                Ok((
                    source,
                    Child::TextPlugin(TextPlugin {
                        attributes: vec![],
                        bounds: "full".to_string(),
                        template: "default".to_string(),
                        text: Some(text.as_str().trim().to_string()),
                        r#type: r#type.to_string(),
                    }),
                ))
            } else {
                // TODO: Figure out how to pass the actual error kind
                // hear instead of hard coding to TakeUntil
                Err(Err::Error(Error::new(source, ErrorKind::TakeUntil)))
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
    fn call_with_no_params() {
        let source = "\n\n-- hr";
        let config = Config::site1_config();
        let r#type = "random-color-square";
        let attributes = vec![];
        let target_text = "#1b3184".to_string();
        let left = Ok((
            "-- hr",
            Child::TextPlugin(TextPlugin {
                attributes: vec![],
                bounds: "full".to_string(),
                template: "default".to_string(),
                text: Some(target_text),
                r#type: "random-color-square".to_string(),
            }),
        ));
        let right = text_plugin_section_full(source, r#type, &attributes, &config);
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn call_with_params() {
        let source = "\n\n-- p";
        let config = Config::site1_config();
        let r#type = "random-color-square";
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
        let target_text = "#236f21".to_string();
        let left = Ok((
            "-- p",
            Child::TextPlugin(TextPlugin {
                attributes: vec![],
                bounds: "full".to_string(),
                template: "default".to_string(),
                text: Some(target_text),
                r#type: "random-color-square".to_string(),
            }),
        ));
        let right = text_plugin_section_full(source, r#type, &attributes, &config);
        assert_eq!(left, right);
    }
}
