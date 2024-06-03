use neopoligengine::span_v39::tokens::*;
use pretty_assertions::assert_eq;

#[test]
fn escaped_close_bracket_basic() {
    let source = "\\]x";
    let left = (
        "x",
        SpanTokenV39 {
            source_text: "\\]".to_string(),
            parsed_text: "]".to_string(),
            kind: SpanTokenV39Kind::EscapedCloseBracket,
        },
    );
    let right = token_escaped_close_bracket_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn escaped_open_bracket_basic() {
    let source = "\\[x";
    let left = (
        "x",
        SpanTokenV39 {
            source_text: "\\[".to_string(),
            parsed_text: "[".to_string(),
            kind: SpanTokenV39Kind::EscapedOpenBracket,
        },
    );
    let right = token_escaped_open_bracket_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn single_close_bracket_basic() {
    let source = "]x";
    let left = (
        "x",
        SpanTokenV39 {
            source_text: "]".to_string(),
            parsed_text: "]".to_string(),
            kind: SpanTokenV39Kind::SingleCloseBracket,
        },
    );
    let right = token_single_close_bracket_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn single_open_bracket_basic() {
    let source = "[x";
    let left = (
        "x",
        SpanTokenV39 {
            source_text: "[".to_string(),
            parsed_text: "[".to_string(),
            kind: SpanTokenV39Kind::SingleOpenBracket,
        },
    );
    let right = token_single_open_bracket_v39(source).unwrap();
    assert_eq!(left, right);
}

// TODO: Rename these to just token (i.e. drop shorthand)

// #[test]
// fn shorthand_token_escaped_backtick_basic() {
//     let source = "\\```";
//     let left = (
//         "``",
//         SpanShorthandTokenV39 {
//             source_text: "\\`".to_string(),
//             parsed_text: "`".to_string(),
//             kind: SpanShorthandTokenV39Kind::EscapedBacktick,
//         },
//     );
//     let right = shorthand_token_escaped_backtick_v39(source).unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn shorthand_token_escaped_colon_basic() {
//     let source = "\\:";
//     let left = (
//         "",
//         SpanShorthandTokenV39 {
//             source_text: "\\:".to_string(),
//             parsed_text: ":".to_string(),
//             kind: SpanShorthandTokenV39Kind::EscapedColon,
//         },
//     );
//     let right = shorthand_token_escaped_colon_v39(source).unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn shorthand_token_escaped_pipe_basic() {
//     let source = "\\|";
//     let left = (
//         "",
//         SpanShorthandTokenV39 {
//             source_text: "\\|".to_string(),
//             parsed_text: "|".to_string(),
//             kind: SpanShorthandTokenV39Kind::EscapedPipe,
//         },
//     );
//     let right = shorthand_token_escaped_pipe_v39(source).unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn shorthand_token_escaped_backslash_basic() {
//     let source = "\\\\";
//     let left = (
//         "",
//         SpanShorthandTokenV39 {
//             source_text: "\\\\".to_string(),
//             parsed_text: "\\".to_string(),
//             kind: SpanShorthandTokenV39Kind::EscapedBackslash,
//         },
//     );
//     let right = shorthand_token_escaped_backslash_v39(source).unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn shorthand_token_single_backtick_test() {
//     let source = "` ";
//     let left = (
//         " ",
//         SpanShorthandTokenV39 {
//             source_text: "`".to_string(),
//             parsed_text: "`".to_string(),
//             kind: SpanShorthandTokenV39Kind::SingleBacktick,
//         },
//     );
//     let right = shorthand_token_single_backtick_v39(source).unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn shorthand_token_single_backslash_test() {
//     let source = "\\ ";
//     let left = (
//         " ",
//         SpanShorthandTokenV39 {
//             source_text: "\\".to_string(),
//             parsed_text: "\\".to_string(),
//             kind: SpanShorthandTokenV39Kind::SingleBackslash,
//         },
//     );
//     let right = shorthand_token_single_backslash_v39(source).unwrap();
//     assert_eq!(left, right);
// }
