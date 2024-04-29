use crate::child::Child;
use crate::config::Config;
use crate::span::span;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many0;
use nom::IResult;

pub fn block<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Child> {
    let (source, _) = multispace0(source)?;
    let (source, _) = not(tag("--"))(source)?;
    let (source, response) = many0(|src| span(src, config))(source)?;
    let (source, _) = alt((tag("\n"), eof))(source)?;
    Ok((source, Child::Block(response)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_block() {
        let source = "alfa bravo";
        let config = Config::set1();
        let left = Ok((
            "",
            Child::Block(vec![
                Span::WordSegment {
                    text: "alfa".to_string(),
                    template: "spans/word_segment.neojinja".to_string(),
                },
                Span::Space {
                    text: " ".to_string(),
                    template: "spans/space.neojinja".to_string(),
                },
                Span::WordSegment {
                    text: "bravo".to_string(),
                    template: "spans/word_segment.neojinja".to_string(),
                },
            ]),
        ));
        let right = block(source, &config);
        assert_eq!(left, right);
    }

    #[test]
    fn word_with_footnote_marker_in_middle() {
        let source = r#"del\^ta"#;
        let config = Config::set1();
        let left = Ok((
            "",
            Child::Block(vec![
                Span::WordSegment {
                    text: "del".to_string(),
                    template: "spans/word_segment.neojinja".to_string(),
                },
                Span::EscapedChar {
                    text: "^".to_string(),
                    template: "spans/escaped_char.neojinja".to_string(),
                },
                Span::WordSegment {
                    text: "ta".to_string(),
                    template: "spans/word_segment.neojinja".to_string(),
                },
            ]),
        ));
        let right = block(source, &config);
        assert_eq!(left, right);
    }
}
