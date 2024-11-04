use crate::span_v42::plaintext::PlainTextSpan;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn plaintext_base_v42(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, results) = many1(alt((
        is_not(" \t\r\n~`!#^*[]{}<>_|:\\"),
        pair(tag("`"), not(tag("`"))).map(|x| x.0),
        pair(tag("~"), not(tag("~"))).map(|x| x.0),
        pair(tag("!"), not(tag("!"))).map(|x| x.0),
        pair(tag("#"), not(tag("#"))).map(|x| x.0),
        pair(tag("^"), not(tag("^"))).map(|x| x.0),
        pair(tag("*"), not(tag("*"))).map(|x| x.0),
        pair(tag("["), not(tag("["))).map(|x| x.0),
        pair(tag("]"), not(tag("]"))).map(|x| x.0),
        pair(tag("{"), not(tag("{"))).map(|x| x.0),
        pair(tag("}"), not(tag("}"))).map(|x| x.0),
        pair(tag("<"), not(tag("<"))).map(|x| x.0),
        pair(tag(">"), not(tag(">"))).map(|x| x.0),
        pair(tag("_"), not(tag("_"))).map(|x| x.0),
    )))
    .context("")
    .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: results.join("").to_string(),
        }),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("alfa", "")]
    #[case("alfa`bravo", "")]
    #[case("alfa~bravo", "")]
    #[case("alfa!bravo", "")]
    #[case("alfa#bravo", "")]
    #[case("alfa^bravo", "")]
    #[case("alfa*bravo", "")]
    #[case("alfa[bravo", "")]
    #[case("alfa]bravo", "")]
    #[case("alfa{bravo", "")]
    #[case("alfa}bravo", "")]
    #[case("alfa<bravo", "")]
    #[case("alfa>bravo", "")]
    #[case("alfa_bravo", "")]
    fn valid_cases_with_remainders(#[case] source: &str, #[case] remainder: &str) {
        let results = plaintext_base_v42(source);
        match results {
            Ok(result) => {
                if result.0 == remainder {
                    assert!(true);
                } else {
                    assert!(
                        false,
                        "Remainder should have been '{}' but got '{}'",
                        &remainder, result.0
                    );
                }
            }
            Err(e) => {
                assert!(false, "{}", e)
            }
        }
    }

    // #[rstest]
    // #[case("")]
    // fn test_invalid_cases(#[case] source: &str) {
    //     let results = plaintext_base_v42(source);
    //     match results {
    //         Ok(_) => {
    //             assert!(false, "This should not have parsed but it did: {}", source);
    //         }
    //         Err(_) => {
    //             assert!(true);
    //         }
    //     }
    // }
}
