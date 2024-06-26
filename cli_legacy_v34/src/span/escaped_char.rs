use crate::span::Span;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;

pub fn escaped_char(source: &str) -> IResult<&str, Span> {
    let (source, _) = tag(r"\")(source)?;
    let (source, result) = alt((tag(r"^"), tag("*")))(source)?;
    Ok((
        source,
        Span::EscapedChar {
            text: result.to_string(),
            template: "spans/escaped_char.neojinja".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    // use super::*;
    // use pretty_assertions::assert_eq;
    // use nom::error::Error;
    // use nom::error::ErrorKind;
    // use nom::Err;

    // #[test]
    // fn escaped_pipe_test() {
    //     let source = r"\|";
    //     let left = Ok((
    //         "",
    //         Span::EscapedPipe {
    //             text: r"\|".to_string(),
    //             template: "spans/escaped_pipe.neojinja".to_string(),
    //         },
    //     ));
    //     let right = escaped_pipe(source);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn dont_pick_up_span_tags() {
    //     let source = ">>";
    //     let left = Err(Err::Error(Error{input: ">", code: ErrorKind::Not}));
    //     let right = greater_than(source);
    //     assert_eq!(left, right);
    // }
}
