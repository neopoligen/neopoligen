use crate::span::Span;
use nom::IResult;
use nom::character::complete::space1;

pub fn space(source: &str) -> IResult<&str, Span> {
    let (source, result) = space1(source)?;
    Ok((source, Span::Space { text: result.to_string(), template: "spans/space.neojinja".to_string() }))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn one_space() {
        let source = " ";
        let left = Span::Space { text: " ".to_string(), template: "spans/space.neojinja".to_string() };
        let right = space(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn multiple_whitespace() {
        let source = "   ";
        let left = Span::Space { text: "   ".to_string(), template: "spans/space.neojinja".to_string() };
        let right = space(source).unwrap().1;
        assert_eq!(left, right);
    }
}