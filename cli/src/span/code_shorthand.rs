use self::any_asterisk::any_asterisk;
use self::any_caret::any_caret;
use self::any_colon::any_colon;
use self::any_hyphen::any_hyphen;
use self::any_lessthan::any_lessthan;
use self::any_newline::any_newline;
use self::any_underscore::any_underscore;
use self::colon_followed_by_multispace1::colon_followed_by_multispace1;
use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn code_shorthand_attr_key_base_v42(source: &str) -> IResult<&str, Vec<Span>, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, spans) = many1(alt((
        plaintext,
        any_hyphen,
        any_lessthan,
        any_caret,
        any_asterisk,
        escaped_backtick,
        single_backtick,
        any_underscore,
        any_hyphen,
        escaped_pipe,
        escaped_colon,
        escaped_backslash,
        colon_not_followed_by_space,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = colon_followed_by_multispace1.context("").parse(source)?;
    Ok((source, spans))
}

pub fn code_shorthand_attr_value_base_v42(
    source: &str,
) -> IResult<&str, Vec<Span>, ErrorTree<&str>> {
    let (source, spans) = many1(alt((
        plaintext,
        any_colon,
        any_hyphen,
        any_caret,
        any_lessthan,
        any_asterisk,
        any_underscore,
        single_backtick,
        escaped_backtick,
        escaped_backslash,
        escaped_colon,
        escaped_pipe,
        any_newline,
    )))
    .context("")
    .parse(source)?;
    Ok((source, spans))
}

pub fn code_shorthand_attr_v42(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, key_spans) = code_shorthand_attr_key_base_v42.context("").parse(source)?;
    let (source, value_spans) = code_shorthand_attr_value_base_v42
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanAttr {
            kind: SpanAttrKind::KeyValueSpansV42 {
                key: key_spans,
                value: value_spans,
            },
        },
    ))
}

pub fn code_shorthand_flag_base_span_v42(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, result) = is_not(":|\\`").context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
            kind: SpanKind::PlainText,
            parsed_text: result.to_string(),
        },
    ))
}

pub fn code_shorthand_flag_v42(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, span_hits) = many1(pair(
        alt((
            code_shorthand_flag_base_span_v42,
            colon_not_followed_by_space,
            escaped_pipe,
            escaped_backslash,
            escaped_backtick,
            single_backtick,
        )),
        not(tag(": ")),
    ))
    .context("")
    .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let spans = span_hits.iter().map(|sh| sh.0.clone()).collect();
    let flag = SpanAttr {
        kind: SpanAttrKind::FlagV42 { value: spans },
    };
    Ok((source, flag))
}

pub fn code_shorthand_base_span(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, span) = alt((
        plaintext.context(""),
        single_backtick,
        any_asterisk,
        any_caret,
        any_hyphen,
        any_lessthan,
        any_newline,
        any_underscore,
        escaped_backtick,
        escaped_backslash,
        escaped_pipe,
        any_colon,
    ))
    .parse(source)?;
    Ok((source, span))
}

pub fn code_shorthand(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = tag("``").context("").context("").parse(source)?;
    let (source, spans) = many1(code_shorthand_base_span).context("").parse(source)?;
    let (source, attrs) = many0(alt((code_shorthand_attr_v42, code_shorthand_flag_v42)))
        .context("")
        .parse(source)?;
    let (source, _) = tag("``").context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs,
            kind: SpanKind::CodeShorthand { children: spans },
            parsed_text: "".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn ping_code_key_value_attr_basic() {
        let source = "|sierra: foxtrot";
        let left = (
            "",
            SpanAttr {
                kind: SpanAttrKind::KeyValueSpansV42 {
                    key: vec![Span {
                        attrs: vec![],
                        kind: SpanKind::PlainText,
                        parsed_text: "sierra".to_string(),
                    }],
                    value: vec![Span {
                        attrs: vec![],
                        kind: SpanKind::PlainText,
                        parsed_text: "foxtrot".to_string(),
                    }],
                },
            },
        );
        let right = code_shorthand_attr_v42(source).unwrap();
        assert_eq!(left, right)
    }

    #[test]
    fn ping_code_value_base_basic_test() {
        let source = "whiskey";
        let left = (
            "",
            vec![Span {
                attrs: vec![],
                kind: SpanKind::PlainText,
                parsed_text: "whiskey".to_string(),
            }],
        );
        let right = code_shorthand_attr_value_base_v42(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn ping_code_key_base_basic_test() {
        let source = "|alfa: bravo";
        let left = (
            "bravo",
            vec![Span {
                attrs: vec![],
                kind: SpanKind::PlainText,
                parsed_text: "alfa".to_string(),
            }],
        );
        let right = code_shorthand_attr_key_base_v42(source).unwrap();
        assert_eq!(left, right);
    }

    // Valid Code Shorthand Key Base
    // \n<^*`_-|:\\
    #[rstest]
    #[case("|spaced are allowed: ")]
    #[case("|colon:without:spaces: ")]
    #[case("|alfa: ")]
    #[case("|bravo-charlie: ")]
    #[case("|\ndelta: ")]
    #[case("|\necho:\n")]
    #[case("|fox<trot: ")]
    #[case("|fox<<trot: ")]
    #[case("|go^lf: ")]
    #[case("|go^^lf: ")]
    #[case("|hot*el: ")]
    #[case("|hot**el: ")]
    #[case("|ind`ia: ")]
    #[case("|ind\\``ia: ")]
    #[case("|jul_iet: ")]
    #[case("|jul__iet: ")]
    #[case("|ki-lo: ")]
    #[case("|ki--lo: ")]
    #[case("|li\\|ma: ")]
    #[case("|mi\\:ke: ")]
    #[case("|mi\\::ke: ")]
    #[case("|mi::ke: ")]
    #[case("|nov\\\\ember: ")]
    fn ping_valid_code_key_base(#[case] source: &str) {
        let result = code_shorthand_attr_key_base_v42(source);
        if result.is_ok() {
            //dbg!(&result.unwrap().clone());
            assert!(true)
        } else {
            dbg!(&result);
            assert!(false);
        }
    }

    // Valid Code Shorthand Attr Value Base
    // \n<^*`_-|:\\
    #[rstest]
    #[case("spaced are allowed|")]
    #[case("colon:without:spaces|")]
    #[case("alfa: |")]
    #[case("bravo-charlie: |")]
    #[case("  leading spaces|")]
    #[case("\nleading newline|")]
    #[case("includes\nnewline|")]
    #[case("fox<trot: |")]
    #[case("fox<<trot: |")]
    #[case("go^lf: |")]
    #[case("go^^lf: |")]
    #[case("hot*el: |")]
    #[case("hot**el: |")]
    #[case("ind`ia: |")]
    #[case("ind\\``ia: |")]
    #[case("jul_iet: |")]
    #[case("jul__iet: |")]
    #[case("ki-lo: |")]
    #[case("ki--lo: |")]
    #[case("li\\|ma: |")]
    #[case("mi\\:ke: |")]
    #[case("mi\\::ke: |")]
    #[case("mi::ke: |")]
    #[case("nov\\\\ember: |")]
    fn ping_valid_code_value_base(#[case] source: &str) {
        let result = code_shorthand_attr_value_base_v42(source);
        let remainder = result.as_ref().unwrap().0;
        if result.is_ok() && remainder == "|" {
            assert!(true)
        } else {
            dbg!(&result);
            assert!(false);
        }
    }

    // Valid Code Shorthands
    #[rstest]
    #[case("``alfa``")]
    #[case("``bravo charlie``")]
    #[case("``delta echo|foxtrot``")]
    #[case("``golf| hotel | india``")]
    #[case("``contains \n neline``")]
    #[case("``contains \n\n nelines``")]
    #[case("``single < lessthan``")]
    #[case("``multiple << lessthans``")]
    #[case("``html <div> tag``")]
    #[case("``single ^ caret``")]
    #[case("``multiple ^^ carets``")]
    #[case("``single * asterisk``")]
    #[case("``multiple ** asterisks``")]
    #[case("``single _ underscore``")]
    #[case("``multiple __ underscores``")]
    #[case("``single - hyphen``")]
    #[case("``multiple -- hyphens``")]
    #[case("``single ` backtick``")]
    #[case("``single \\` backtick esaped``")]
    #[case("``escaped \\`` backtick before single backtick``")]
    #[case("``escaped \\| pipe``")]
    #[case("``escaped \\\\ backslash``")]
    #[case("``bravo:charlie``")]
    #[case("``bravo: charlie``")]
    #[case("``bravo : charlie``")]
    fn ping_valid_code_shorthands(#[case] source: &str) {
        let result = code_shorthand(source);
        if result.is_ok() {
            assert!(true)
        } else {
            dbg!(&result);
            assert!(false);
        }
    }

    // Invalid Code Shorthands
    #[rstest]
    #[case("``no trailing pipe|``")]
    fn ping_invalid_code_shorthands(#[case] source: &str) {
        let result = code_shorthand(source);
        if result.is_ok() {
            dbg!(&result);
            assert!(false);
        } else {
            assert!(true)
        }
    }

    // Valid Code Shorthand Flags
    #[rstest]
    #[case("|alfa")]
    #[case("| bravo ")]
    #[case("|space in flag")]
    #[case("|newline\nin flag")]
    #[case("| charlie-delta ")]
    #[case("|https://www.example.com/")]
    #[case("|\ncontains")]
    #[case("|single`backtick")]
    #[case("|esacped\\`backtick")]
    #[case("|esacped\\|pipe")]
    #[case("|esacped\\\\escape")]
    fn ping_valid_code_shorthand_flags(#[case] source: &str) {
        let result = code_shorthand_flag_v42(source);
        if result.as_ref().is_ok() {
            if result.as_ref().unwrap().0 == "" {
                assert!(true)
            } else {
                dbg!(format!("missed: {}", result.unwrap().0));
                assert!(false);
            }
        } else {
            dbg!(&result);
            assert!(false);
        }
    }

    // Invalid Code Shorthand Flags
    #[rstest]
    #[case("|alfa: value")]
    fn ping_invalid_code_shorthand_flags(#[case] source: &str) {
        let result = code_shorthand_flag_v42(source);
        if result.as_ref().is_ok() {
            dbg!(&result);
            assert!(false);
        } else {
            assert!(true)
        }
    }

    #[test]
    fn ping_code_shorthand_with_flag_attr() {
        let source = "``code|rust``";
        let left = (
            "",
            Span {
                attrs: vec![SpanAttr {
                    kind: SpanAttrKind::FlagV42 {
                        value: vec![Span {
                            attrs: vec![],
                            kind: SpanKind::PlainText,
                            parsed_text: "rust".to_string(),
                        }],
                    },
                }],
                parsed_text: "".to_string(),
                kind: SpanKind::CodeShorthand {
                    children: vec![Span {
                        attrs: vec![],
                        kind: SpanKind::PlainText,
                        parsed_text: "code".to_string(),
                    }],
                },
            },
        );
        let result = code_shorthand(source);
        let right = result.unwrap();
        assert_eq!(left, right);
    }

    #[rstest]
    #[case("``alfa``", 0, "single word")]
    #[case("``alfa bravo``", 0, "space in text")]
    #[case("``alfa-bravo``", 0, "hyphen in text")]
    #[case("``alfa`bravo``", 0, "single backtick in text")]
    #[case("``alfa_bravo``", 0, "with single underscore")]
    #[case("``alfa\\`bravo``", 0, "escaped backtick in text")]
    #[case("``alfa\\|bravo``", 0, "escaped pipe in text")]
    #[case("``alfa\\\\bravo``", 0, "escaped backslash in text")]
    #[case("``alfa:bravo``", 0, "colon in text")]
    #[case("``alfa: bravo``", 0, "colon in text before space")]
    #[case("``alfa :bravo``", 0, "colon in text after space")]
    #[case("``alfa\\|bravo``", 0, "escaped pipe in text")]
    #[case("``alfa\\`bravo``", 0, "escaped backtick in text")]
    #[case("``alfa|bravo``", 1, "single flag attr")]
    #[case("``alfa|bravo charlie``", 1, "space in flag")]
    #[case("``alfa|bravo`charlie``", 1, "single backtick in flag")]
    #[case("``alfa|bravo\ncharlie``", 1, "newline in flag")]
    #[case("``alfa|bravo\\|charlie``", 1, "escaped pipe in flag")]
    #[case("``alfa|bravo\\`charlie``", 1, "escaped backtick in flag")]
    #[case("``alfa|bravo\\\\charlie``", 1, "escaped baskslash in flag")]
    #[case("``alfa|bravo|charlie``", 2, "two flag attrs")]
    #[case("``alfa|bravo: charlie``", 1, "single key value attr")]
    #[case("``alfa|bravo: charlie|delta: echo``", 2, "single key value attr")]
    #[case("``\nalfa\n|\nbravo\n``", 1, "newlines in shorthand")]
    #[case(
        "``\nalfa\n|\nbravo\n|\ncharlie\n``",
        2,
        "newlines in shorthand multiple attrs"
    )]
    #[case("``Result<(), Box<dyn std::error::Error>>``", 0, "rust example")]
    fn ping_code_shorhand_fixture(
        #[case] input: &str,
        #[case] attrs: usize,
        #[case] _description: &str,
    ) {
        let (remainder, span) = code_shorthand(input).unwrap();
        assert_eq!(remainder, "");
        assert_eq!(span.attrs.len(), attrs);
    }

    #[rstest]
    #[case("``\\``", "can't have a single backslash at the end of code text")]
    fn ping_error_cases(#[case] input: &str, #[case] _desc: &str) {
        assert!(code_shorthand(input).is_err());
    }
}
