pub mod section_flag_attribute;
pub mod section_key_value_attribute;

use crate::section_attribute::section_flag_attribute::*;
use crate::section_attribute::section_key_value_attribute::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::IResult;
use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(content = "content", rename_all = "lowercase", tag = "type")]
pub enum SectionAttribute {
    KeyValue { key: String, value: String },
    Bool { key: String },
    TemporaryNoneForDev,
}

pub fn section_attribute(source: &str) -> IResult<&str, SectionAttribute> {
    let (source, _) = tag("--")(source)?;
    let (source, _) = space1(source)?;
    let (source, attribute) = alt((section_flag_attribute, section_key_value_attribute))(source)?;
    Ok((source, attribute))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::error::Error;
    use nom::error::ErrorKind;
    use nom::Err;
    use pretty_assertions::assert_eq;

    #[test]
    fn section_attribute_integration_no_attributes() {
        let source = "\nTitle Echo\n\n--p";
        let left = Err(Err::Error(Error::new(
            "\nTitle Echo\n\n--p",
            ErrorKind::Tag,
        )));
        let right = section_attribute(source);
        assert_eq!(left, right);
    }

    #[test]
    fn section_attribute_integration_found_key_value() {
        let source = "-- oscar: tango\n--p";
        let left = Ok((
            "--p",
            SectionAttribute::KeyValue {
                key: "oscar".to_string(),
                value: "tango".to_string(),
            },
        ));
        let right = section_attribute(source);
        assert_eq!(left, right);
    }

    // TODO: boolean test
}
