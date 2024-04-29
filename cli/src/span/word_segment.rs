use crate::span::Span;
use nom::bytes::complete::is_not;
use nom::IResult;

pub fn word_segment(source: &str) -> IResult<&str, Span> {
    let (source, content) = is_not(" \\\n\t|<>^`_*")(source)?;
    Ok((
        source,
        Span::WordSegment {
            text: content.to_string(),
            template: "spans/word_segment.neojinja".to_string(),
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
                template: "spans/word_segment.neojinja".to_string(),
            },
        ));
        let right = word_segment(source);
        assert_eq!(left, right);
    }

    #[test]
    fn dash_word() {
        let source = "- ";
        let left = Ok((
            " ",
            Span::WordSegment {
                text: "-".to_string(),
                template: "spans/word_segment.neojinja".to_string(),
            },
        ));
        let right = word_segment(source);
        assert_eq!(left, right);
    }
}
