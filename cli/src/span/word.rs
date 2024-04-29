use crate::span::Span;
use nom::bytes::complete::is_not;
use nom::IResult;

pub fn word(source: &str) -> IResult<&str, Span> {
    let (source, content) = is_not(" \n\t|<>^")(source)?;
    Ok((
        source,
        Span::WordSegment {
            text: content.to_string(),
            template: "spans/word.neojinja".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_word() {
        let source = "delta ";
        let left = Ok((
            " ",
            Span::WordSegment {
                text: "delta".to_string(),
                template: "spans/word.neojinja".to_string(),
            },
        ));
        let right = word(source);
        assert_eq!(left, right);
    }

    #[test]
    fn dash_word() {
        let source = "- ";
        let left = Ok((
            " ",
            Span::WordSegment {
                text: "-".to_string(),
                template: "spans/word.neojinja".to_string(),
            },
        ));
        let right = word(source);
        assert_eq!(left, right);
    }
}
