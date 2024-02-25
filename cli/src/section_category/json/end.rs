use crate::child::child;
use crate::child::Child;
use crate::config::Config;
use crate::section::Section;
use crate::section_category::SectionCategory;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn json_section_end<'a>(
    source: &'a str,
    r#type: &str,
    key_value_attributes: BTreeMap<String, String>,
    flag_attributes: BTreeSet<String>,
    config: &'a Config,
    initial_source: &str,
) -> IResult<&'a str, Child> {
    if config
        .section_categories
        .json
        .contains(&r#type.to_string())
    {
        let (source, containers) = many0(|src| child(src, config))(source)?;
        let (source, _) = tuple((space0, line_ending))(source)?;
        let (source, _) = multispace0(source)?;
        let section = Child::Section(Section {
            key_value_attributes,
            flag_attributes,
            bounds: "end".to_string(),
            category: SectionCategory::JsonSectionEnd { containers },
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
    use crate::span::Span;
    use pretty_assertions::assert_eq;

    #[test]
    // #[ignore]
    fn preformatted_section_end_integration() {
        let src = "papa mike\n\n-- div";
        let r#type = "metadata";
        let key_value_attributes =  BTreeMap::new();
        let flag_attributes = BTreeSet::new();
        let config = Config::site1_config();
        let initial_source = "-- /metadata\n\npapa mike\n\n-- div";
        let left = Ok((
            "-- div",
            Child::Section(Section {
                key_value_attributes: key_value_attributes.clone(),
                flag_attributes: flag_attributes.clone(),
                bounds: "end".to_string(),
                category: SectionCategory::JsonSectionEnd {
                    containers: vec![Child::Block(vec![
                        Span::Word {
                            text: "papa".to_string(),
                            template: "spans/word.jinja".to_string(),
                        },
                        Span::Space {
                            text: " ".to_string(),
                            template: "spans/space.jinja".to_string(),
                        },
                        Span::Word {
                            text: "mike".to_string(),
                            template: "spans/word.jinja".to_string(),
                        },
                    ])],
                },
                template: "default".to_string(),
                r#type: "metadata".to_string(),
                source: "-- /metadata\n\npapa mike".to_string(),
            }),
        ));
        let right = json_section_end(
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
