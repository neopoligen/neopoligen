// use neopoligengine::site_config::SiteConfig;
use neopoligengine::span_attr_v39::*;
use neopoligengine::span_shorthand_token_v39::*;
use neopoligengine::span_v39::code_shorthand::*;
use neopoligengine::span_v39::*;
// use nom::multi::many1;
// use nom::Parser;
use pretty_assertions::assert_eq;

#[test]
fn shorthand_token_escaped_backtick_basic() {
    let source = "\\```";
    let left = (
        "``",
        SpanShorthandTokenV39 {
            kind: SpanShorthandTokenV39Kind::EscapedBacktick {
                source_text: "\\`".to_string(),
                parsed_text: "`".to_string(),
            },
        },
    );
    let right = shorthand_token_escaped_backtick_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn shorthand_token_escaped_pipe_basic() {
    let source = "\\|";
    let left = (
        "",
        SpanShorthandTokenV39 {
            kind: SpanShorthandTokenV39Kind::EscapedPipe {
                source_text: "\\|".to_string(),
                parsed_text: "|".to_string(),
            },
        },
    );
    let right = shorthand_token_escaped_pipe_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn shorthand_token_escaped_slash_basic() {
    let source = "\\\\";
    let left = (
        "",
        SpanShorthandTokenV39 {
            kind: SpanShorthandTokenV39Kind::EscapedSlash {
                source_text: "\\\\".to_string(),
                parsed_text: "\\".to_string(),
            },
        },
    );
    let right = shorthand_token_escaped_slash_v39(source).unwrap();
    assert_eq!(left, right);
}
