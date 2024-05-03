use crate::span::Span;
use nom::combinator::not;
use nom::IResult;
use nom::bytes::complete::tag;

pub fn greater_than(source: &str) -> IResult<&str, Span> {
    let (source, result) = tag(">")(source)?;
    not(tag(">"))(source)?;
    Ok((
        source,
        Span::GreaterThan {
            text: result.to_string(),
            template: "spans/less_than.neojinja".to_string(),
        },
    ))
}


#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use nom::error::Error;
    use nom::error::ErrorKind;
    use nom::Err;

    #[test]
    fn single_greater_than() {
        let source = ">";
        let left = Ok((
            "",
            Span::GreaterThan {
                text: ">".to_string(),
                template: "spans/less_than.neojinja".to_string(),
            },
        ));
        let right = greater_than(source);
        assert_eq!(left, right);
    }

    #[test]
    fn dont_pick_up_span_tags() {
        let source = ">>";
        let left = Err(Err::Error(Error{input: ">", code: ErrorKind::Not}));
        let right = greater_than(source);
        assert_eq!(left, right);
    }

}
