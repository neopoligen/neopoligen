use crate::config::Config;
use crate::span::span;
use crate::span::Span;
use nom::bytes::complete::tag;
// use nom::bytes::complete::tag_no_case;
// use nom::bytes::complete::take_until;
// use nom::error::Error;
// use nom::error::ErrorKind;
use nom::multi::many1;
// use nom::Err;
use nom::IResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn code<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Span> {
    let (source, _) = tag("`")(source)?;
    let (source, content) = many1(|src| span(src, config))(source)?;
    let (source, _) = tag("``")(source)?;

    // let (source, span_type) = match match config.standard_spans.iter().find_map(|t| {
    //     match tag_no_case::<&str, &str, Error<&str>>(t.as_str())(source) {
    //         Ok(x) => Some(x),
    //         Err(_) => None,
    //     }
    // }) {
    //     Some(x) => Ok(x),
    //     None => Err(Err::Error(Error {
    //         input: "\n",
    //         code: ErrorKind::Tag,
    //     })),
    // } {
    //     Ok(x) => Ok(x),
    //     Err(_) => Err(Err::Error(Error::new(source, ErrorKind::Tag))),
    // }?;
    // let (source, _) = tag("|")(source)?;
    // let (source, content) = many1(|src| span(src, config))(source)?;
    // let (source, _) = tag(">>")(source)?;

    Ok((
        source,
        Span::Code {
            spans: content,
            key_value_attributes: BTreeMap::new(),
            flag_attributes: BTreeSet::new(),
            span_type: "code".to_string(),
            template: format!("spans/code.neojinja"),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_code() {
        let source = "`example``";
        let config = Config::set1();
        let left = Ok((
            "",
            Span::Code {
                span_type: "code".to_string(),
                spans: vec![Span::WordSegment {
                    text: "example".to_string(),
                    template: "spans/word_segment.neojinja".to_string(),
                }],
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                template: "spans/code.neojinja".to_string(),
            },
        ));
        let right = code(source, &config);
        assert_eq!(left, right);
    }
}
