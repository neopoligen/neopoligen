use crate::section_attribute::SectionAttribute;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::not_line_ending;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::IResult;

pub fn section_key_value_attribute(source: &str) -> IResult<&str, SectionAttribute> {
    let (source, _) = not(tag("\n"))(source)?;
    let (source, key) = is_not(" :\n")(source)?;
    let (source, _) = tag(":")(source)?;
    let (source, _) = space1(source)?;
    let (source, value) = not_line_ending(source)?;
    let (source, _) = alt((tag("\n"), eof))(source)?;
    Ok((
        source,
        SectionAttribute::KeyValue {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::error::Error;
    use nom::error::ErrorKind;
    use nom::Err;
    use pretty_assertions::assert_eq;

    #[test]
    fn section_key_value_attribute_no_attributes() {
        let source = "\nTitle Echo\n\n--p";
        let left = Err(Err::Error(Error::new(
            "\nTitle Echo\n\n--p",
            ErrorKind::Not,
        )));
        let right = section_key_value_attribute(source);
        assert_eq!(left, right);
    }

    #[test]
    fn section_key_value_attribute_found_an_attribute() {
        let source = "delta: papa\n--p";
        let left = Ok((
            "--p",
            SectionAttribute::KeyValue {
                key: "delta".to_string(),
                value: "papa".to_string(),
            },
        ));
        let right = section_key_value_attribute(source);
        assert_eq!(left, right);
    }

    #[test]
    fn section_key_value_attribute_at_end_of_file() {
        let source = "tango: sierra";
        let left = Ok((
            "",
            SectionAttribute::KeyValue {
                key: "tango".to_string(),
                value: "sierra".to_string(),
            },
        ));
        let right = section_key_value_attribute(source);
        assert_eq!(left, right);
    }
}
