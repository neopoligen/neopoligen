use crate::config::Config;
use crate::span::span;
use crate::span::Span;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::many1;
use nom::Err;
use nom::IResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn footnote<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Span> {
    let (source, _) = tag_no_case("^example^^")(source)?;
    let (source, content_to_parse) = take_until("^")(source)?;
    let (source, _) = tag("^^")(source)?;

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
        Span::StandardSpan {
            spans: vec![Span::Word {
                text: "example".to_string(),
                template: "spans/word.neojinja".to_string(),
            }],
            key_value_attributes: BTreeMap::new(),
            flag_attributes: BTreeSet::new(),
            span_type: "footnote".to_string(),
            template: format!("spans/footnote.neojinja"),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[ignore]
    fn basic_footnote() {
        let source = "^example^^";
        let config = Config::set1();
        let left = Ok((
            "",
            Span::StandardSpan {
                span_type: "footnote".to_string(),
                spans: vec![Span::Word {
                    text: "example".to_string(),
                    template: "spans/word.neojinja".to_string(),
                }],
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                template: "spans/footnote.neojinja".to_string(),
            },
        ));
        let right = footnote(source, &config);
        assert_eq!(left, right);
    }
}
