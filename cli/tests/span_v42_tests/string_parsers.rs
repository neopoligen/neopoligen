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

#[rstest]
#[case("\n", "Don't Trigger On Newline")]
#[case(" \n", "Don't Trigger In Front Of Newline")]
#[case(" ", "Don't Trigger At End Of File")]
fn solo_plain_text_space1_as_single_space_invalid_tests(
    #[case] source: &str,
    #[case] description: &str,
) {
    let parsed = plain_text_space1_as_single_space(source);
    assert!(parsed.is_err(), "{}", description);
}

// NOT SURE IF THIS IS NEEDED
// #[rstest]
// #[case("", "", "And End Of File")]
// #[case(" ", "", "Space And End Of File")]
// #[case("\n", "", "Line Ending And End Of File")]
// #[case(" \n\n", "\n", "Line Ending And End Of File")]
// fn solo_plain_text_end_of_block_valid(
//     #[case] source: &str,
//     #[case] remainder: &str,
//     #[case] description: &str,
// ) {
//     let left = remainder;
//     let right = plain_text_end_of_block(source).unwrap().1;
//     assert_eq!(left, right, "{}", description);
// }

// NOT SURE IF THIS IS NEEDED
// #[rstest]
// #[case("", "And End Of File")]
// fn solo_plain_text_end_of_block_invalid(#[case] source: &str, #[case] description: &str) {
//     if plain_text_end_of_block(source).is_err() {
//         assert!(true);
//     } else {
//         assert!(false, "##### ERROR ##### {}", description);
//     }
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

#[rstest]
#[case("\n", " ", "")]
fn plain_text_single_line_ending_as_space_valid_tests(
    #[case] source: &str,
    #[case] got: &str,
    #[case] remainder: &str,
) {
    let matcher = (remainder, got);
    let parsed = plain_text_single_line_ending_as_space(source).unwrap();
    assert_eq!(matcher, parsed);
}

#[rstest]
#[case("\n\n", "Two Newlines In A Row")]
#[case("\n  \n", "Two Newlines Separated By Whitespace")]
fn plain_text_single_newline_as_space_invalid_tests(
    #[case] source: &str,
    #[case] description: &str,
) {
    let parsed = plain_text_single_line_ending_as_space(source);
    if parsed.is_err() {
        assert!(true);
    } else {
        assert!(false, "##### ERROR ##### {}", description);
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
