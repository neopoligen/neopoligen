use neopoligengine::span_shorthand_token_v39::*;
use pretty_assertions::assert_eq;

#[test]
fn shorthand_token_escaped_backtick_basic() {
    let source = "\\```";
    let left = (
        "``",
        SpanShorthandTokenV39 {
            source_text: "\\`".to_string(),
            parsed_text: "`".to_string(),
            kind: SpanShorthandTokenV39Kind::EscapedBacktick,
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
            source_text: "\\|".to_string(),
            parsed_text: "|".to_string(),
            kind: SpanShorthandTokenV39Kind::EscapedPipe,
        },
    );
    let right = shorthand_token_escaped_pipe_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn shorthand_token_escaped_backslash_basic() {
    let source = "\\\\";
    let left = (
        "",
        SpanShorthandTokenV39 {
            source_text: "\\\\".to_string(),
            parsed_text: "\\".to_string(),
            kind: SpanShorthandTokenV39Kind::EscapedBackslash,
        },
    );
    let right = shorthand_token_escaped_backslash_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn shorthand_token_single_backtick_test() {
    let source = "` ";
    let left = (
        " ",
        SpanShorthandTokenV39 {
            source_text: "`".to_string(),
            parsed_text: "`".to_string(),
            kind: SpanShorthandTokenV39Kind::SingleBacktick,
        },
    );
    let right = shorthand_token_single_backtick_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn shorthand_token_single_backslash_test() {
    let source = "\\ ";
    let left = (
        " ",
        SpanShorthandTokenV39 {
            source_text: "\\".to_string(),
            parsed_text: "\\".to_string(),
            kind: SpanShorthandTokenV39Kind::SingleBackslash,
        },
    );
    let right = shorthand_token_single_backslash_v39(source).unwrap();
    assert_eq!(left, right);
}
