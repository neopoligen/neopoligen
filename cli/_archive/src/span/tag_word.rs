use crate::span::Span;
use nom::IResult;
use nom::bytes::complete::is_not;

pub fn tag_word(source: &str) -> IResult<&str, Span> {
    let (source, content) = is_not(" \n\t>|")(source)?;
    // let (source, content) = alpha1(source)?;
    Ok((source, Span::Word{text: content.to_string(), template: "spans/word.jinja".to_string()}))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_word() {
        let source = "delta ";
        let left = Ok((" ", Span::Word{text: "delta".to_string(), template: "spans/word.jinja".to_string()}));
        let right = tag_word(source);
        assert_eq!(left, right);
    }

    #[test]
    fn dash_word() {
        let source = "- ";
        let left = Ok((" ",  Span::Word{text: "-".to_string(), template: "spans/word.jinja".to_string()}));
        let right = tag_word(source);
        assert_eq!(left, right);
    }

    #[test]
    fn avoid_gt() {
        let source = "echo>>";
        let left = Ok((">>", Span::Word{text: "echo".to_string(), template: "spans/word.jinja".to_string()}));
        let right = tag_word(source);
        assert_eq!(left, right);
    }

    #[test]
    fn avoid_pipe() {
        let source = "hotel|";
        let left = Ok(("|", Span::Word{text: "hotel".to_string(), template: "spans/word.jinja".to_string()}));
        let right = tag_word(source);
        assert_eq!(left, right);
    }

}