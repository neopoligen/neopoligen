use crate::span::Span;
use nom::IResult;
use nom::bytes::complete::tag;

pub fn pipe_by_itself_in_code(source: &str) -> IResult<&str, Span> {
    let (source, result) = tag(r#"`|``"#)(source)?;
    Ok((
        source,
        Span::PipeByItselfInCode {
            text:
            result.to_string(),
            template: "spans/pipe_by_itself_in_code.jinja".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    // use nom::error::Error;
    // use nom::error::ErrorKind;
    // use nom::Err;

    #[test]
    fn escaped_pipe_test() {
        let source = r#"`|``"#;
        let left = Ok((
            "",
            Span::PipeByItselfInCode {
                text: r#"`|``"#.to_string(),
                template: "spans/pipe_by_itself_in_code.jinja".to_string(),
            },
        ));
        let right = pipe_by_itself_in_code(source);
        assert_eq!(left, right);
    }

}
