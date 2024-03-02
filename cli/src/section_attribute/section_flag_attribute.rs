use crate::section_attribute::SectionAttribute;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::eof;
use nom::combinator::not;
use nom::IResult;

pub fn section_flag_attribute(source: &str) -> IResult<&str, SectionAttribute> {
    let (source, _) = not(tag("\n"))(source)?;
    let (source, key) = is_not(":\n")(source)?;
    let (source, _) = alt((tag("\n"), eof))(source)?;
    Ok((
        source,
        SectionAttribute::Bool {
            key: key.to_string(),
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
    fn section_flag_attribute_no_attributes() {
        let source = "\nSierra Tango\n\n--p";
        let left = Err(Err::Error(Error::new(
            "\nSierra Tango\n\n--p",
            ErrorKind::Not,
        )));
        let right = section_flag_attribute(source);
        assert_eq!(left, right);
    }

    #[test]
    fn section_flag_attribute_found_an_attribute() {
        let source = "echo\n--p";
        let left = Ok((
            "--p",
            SectionAttribute::Bool {
                key: "echo".to_string(),
            },
        ));
        let right = section_flag_attribute(source);
        assert_eq!(left, right);
    }

    #[test]
    fn section_flag_attribute_at_end_of_file() {
        let source = "delta";
        let left = Ok((
            "",
            SectionAttribute::Bool {
                key: "delta".to_string(),
            },
        ));
        let right = section_flag_attribute(source);
        assert_eq!(left, right);
    }
}
