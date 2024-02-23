use nom::IResult;
use nom::branch::alt;
use crate::span::tag_word::tag_word;
use crate::span::space::space;
use crate::span::single_newline::single_newline;
use crate::span::Span;
// use crate::section_attribute::section_attribute::SectionAttribute;


pub fn tag_span(source: &str) -> IResult<&str, Span> {
    let (source, content) = alt((single_newline, space, tag_word))(source)?;
    Ok((source, content))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn single_word_isolated() {
        let source = "alfa";
        let left = Span::Word{text: "alfa".to_string(), template: "spans/word.jinja".to_string()};
        let right = tag_span(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn single_space() {
        let source = " ";
        let left = Span::Space { text: " ".to_string(), template: "spans/space.jinja".to_string() };
        let right = tag_span(source).unwrap().1;
        assert_eq!(left, right);
    }
}
