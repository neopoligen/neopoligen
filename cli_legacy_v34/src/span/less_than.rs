use crate::span::Span;
use nom::combinator::not;
use nom::IResult;
use nom::bytes::complete::tag;

pub fn less_than(source: &str) -> IResult<&str, Span> {
    // dbg!(&source);
    let (source, result) = tag("<")(source)?;
    // dbg!(&source);
    not(tag("<"))(source)?;
    Ok((
        source,
        Span::LessThan {
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
    fn single_less_than() {
        let source = "<";
        let left = Ok((
            "",
            Span::LessThan {
                text: "<".to_string(),
                template: "spans/less_than.neojinja".to_string(),
            },
        ));
        let right = less_than(source);
        assert_eq!(left, right);
    }


    #[test]
    fn with_word_behind() {
        let source = "<delta";
        let left = Ok((
            "delta",
            Span::LessThan {
                text: "<".to_string(),
                template: "spans/less_than.neojinja".to_string(),
            },
        ));
        let right = less_than(source);
        assert_eq!(left, right);
    }

    #[test]
    fn dont_pick_up_span_tags() {
        let source = "<<";
        let left = Err(Err::Error(Error{input: "<", code: ErrorKind::Not}));
        let right = less_than(source);
        assert_eq!(left, right);
    }

}
