use neopoligengine::span_v42::string_parsers::*;
use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
#[case(" foxtrot", "foxtrot")]
#[case("    foxtrot", "foxtrot")]
fn solo_plain_text_space1_as_single_space_valid_tests(
    #[case] source: &str,
    #[case] remainder: &str,
) {
    let matcher = (remainder, " ");
    let parsed = plain_text_space1_as_single_space(source).unwrap();
    assert_eq!(matcher, parsed);
}

// NOTE: Not sure if this is needed, might be able
// to do it just by making sure the spaces don't kick
// past empty lines with only whitespace
// #[rstest]
// #[case("", "And End Of File")]
// #[case(" ", "Space And End Of File")]
// #[case("\n", "Line Ending And End Of File")]
// fn solo_plain_text_end_of_block(#[case] source: &str, #[case] description: &str) {
//     let left = "";
//     let right = plain_text_end_of_block(source).unwrap().1;
//     assert_eq!(left, right, "{}", description);
// }

#[rstest]
#[case("\\:bravo", ":", "bravo")]
#[case("\\|bravo", "|", "bravo")]
#[case("\\`bravo", "`", "bravo")]
#[case("\\~bravo", "~", "bravo")]
#[case("\\!bravo", "!", "bravo")]
#[case("\\#bravo", "#", "bravo")]
#[case("\\^bravo", "^", "bravo")]
#[case("\\*bravo", "*", "bravo")]
#[case("\\[bravo", "[", "bravo")]
#[case("\\]bravo", "]", "bravo")]
#[case("\\{bravo", "{", "bravo")]
#[case("\\}bravo", "}", "bravo")]
#[case("\\<bravo", "<", "bravo")]
#[case("\\>bravo", ">", "bravo")]
#[case("\\_bravo", "_", "bravo")]
fn solo_escaped_character_tests(#[case] source: &str, #[case] got: &str, #[case] remainder: &str) {
    let matcher = (remainder, got);
    let parsed = escaped_character(source).unwrap();
    assert_eq!(matcher, parsed);
}

#[rstest]
#[case("alfa", &["alfa"], "")]
#[case("alfa bravo", &["alfa", " ", "bravo"], "")]
#[case("alfa    bravo", &["alfa", " ", "bravo"], "")]
#[case("alfa\nbravo", &["alfa", " ", "bravo"], "")]
#[case("alfa\n\nbravo", &["alfa"], "\n\nbravo")]
#[case("alfa~bravo", &["alfa", "~", "bravo"], "")]
#[case("alfa|bravo", &["alfa", "|", "bravo"], "")]
#[case("alfa||bravo", &["alfa", "||", "bravo"], "")]
#[case("alfa:bravo", &["alfa", ":", "bravo"], "")]
#[case("alfa::bravo", &["alfa", "::", "bravo"], "")]
#[case("alfa~~bravo~~", &["alfa"], "~~bravo~~")]
#[case("alfa ~~bravo~~", &["alfa", " "], "~~bravo~~")]
fn plain_text_span_for_block(#[case] source: &str, #[case] got: &[&str], #[case] remainder: &str) {
    let matcher = (remainder, got.to_vec());
    let parsed = plain_text_span_for_block_paragraph(source).unwrap();
    assert_eq!(matcher, parsed);
}

#[rstest]
#[case("|", "|", "")]
#[case("||", "||", "")]
fn plain_text_any_pipes_tests(#[case] source: &str, #[case] got: &str, #[case] remainder: &str) {
    let matcher = (remainder, got);
    let parsed = plain_text_any_pipes(source).unwrap();
    assert_eq!(matcher, parsed);
}

#[rstest]
#[case("|", "|", "")]
fn plain_text_single_pipe_tests(#[case] source: &str, #[case] got: &str, #[case] remainder: &str) {
    let matcher = (remainder, got);
    let parsed = plain_text_single_pipe(source).unwrap();
    assert_eq!(matcher, parsed);
}

// #[rstest]
// #[case("alfa", &["alfa"], "")]
// #[case("alfa bravo", &["alfa", " ", "bravo"], "")]
// #[case("alfa bravo\ncharlie", &["alfa", " ", "bravo", " ", "charlie"], "")]
// #[case("alfa bravo\n\ncharlie", &["alfa", " ", "bravo"], "\n\ncharlie")]
// #[case("alfa~bravo", &["alfa", "~", "bravo"], "")]
// fn plain_text_string_with_line_spaces_and_single_newlines_tests(
//     #[case] source: &str,
//     #[case] got: &[&str],
//     #[case] remainder: &str,
// ) {
//     let matcher = (remainder, got.to_vec());
//     let parsed = plain_text_string_with_line_spaces_and_single_newlines(source).unwrap();
//     assert_eq!(matcher, parsed);
// }

#[rstest]
#[case("alfa", "alfa", "")]
#[case("alfa bravo", "alfa", " bravo")]
#[case("alfa\nbravo", "alfa", "\nbravo")]
#[case("`bravo", "`", "bravo")]
#[case("~bravo", "~", "bravo")]
#[case("!bravo", "!", "bravo")]
#[case("#bravo", "#", "bravo")]
#[case("^bravo", "^", "bravo")]
#[case("*bravo", "*", "bravo")]
#[case("[bravo", "[", "bravo")]
#[case("]bravo", "]", "bravo")]
#[case("{bravo", "{", "bravo")]
#[case("}bravo", "}", "bravo")]
#[case("<bravo", "<", "bravo")]
#[case(">bravo", ">", "bravo")]
#[case("_bravo", "_", "bravo")]
fn plain_text_string_base_tests(#[case] source: &str, #[case] got: &str, #[case] remainder: &str) {
    let matcher = (remainder, got);
    let parsed = plain_text_string_base(source).unwrap();
    assert_eq!(matcher, parsed);
}

// #[rstest]
// #[case("alfa", &["alfa"], "")]
// #[case("alfa bravo", &["alfa", " ", "bravo"], "")]
// #[case("alfa bravo\ncharlie", &["alfa", " ", "bravo"], "\ncharlie")]
// fn plain_text_string_with_line_spaces_tests(
//     #[case] source: &str,
//     #[case] got: &[&str],
//     #[case] remainder: &str,
// ) {
//     let matcher = (remainder, got.to_vec());
//     let parsed = plain_text_string_with_line_spaces(source).unwrap();
//     assert_eq!(matcher, parsed);
// }

#[rstest]
#[case("\n", " ", "")]
fn plain_text_single_newline_as_space_tests(
    #[case] source: &str,
    #[case] got: &str,
    #[case] remainder: &str,
) {
    let matcher = (remainder, got);
    let parsed = plain_text_single_newline_as_space(source).unwrap();
    assert_eq!(matcher, parsed);
}

#[rstest]
#[case("\n\n")]
fn invalid_plain_text_single_newline_as_space_tests(#[case] source: &str) {
    let parsed = plain_text_single_newline_as_space(source);
    if parsed.is_err() {
        assert!(true);
    } else {
        assert!(false);
    }
}

// #[rstest]
// #[case(" ", " ", "")]
// fn solo_plain_text_space1_not_newline_tests(
//     #[case] source: &str,
//     #[case] got: &str,
//     #[case] remainder: &str,
// ) {
//     let matcher = (remainder, got);
//     let parsed = plain_text_space1_not_newline(source).unwrap();
//     assert_eq!(matcher, parsed);
// }

//
