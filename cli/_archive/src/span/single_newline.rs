use crate::span::Span;
use nom::IResult;
use nom::character::complete::line_ending;
use nom::combinator::not;

pub fn single_newline(source: &str) -> IResult<&str, Span> {
    let (source, result) = line_ending(source)?;
    not(line_ending)(source)?;
    Ok((source, Span::Space{ text: result.to_string(), template: "spans/space.jinja".to_string()}))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::Err;
    use nom::error::ErrorKind;
    use nom::error::Error;

    #[test]
    fn one_newline_only() {
        let source = "\n";
        let left = Span::Space { text: "\n".to_string(), template: "spans/space.jinja".to_string() };
        let right = single_newline(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn do_not_match_multiple_newlines() {
        let source = "\n\n";
        let left = Err(Err::Error(Error{input: "\n", code: ErrorKind::Not}));
        let right = single_newline(source);
        assert_eq!(left, right);
    }
}