use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn plain_text_string_with_line_spaces_and_single_newlines(
    source: &str,
) -> IResult<&str, Vec<&str>, ErrorTree<&str>> {
    let (source, results) = many1(alt((
        plain_text_string_base,
        space1,
        plain_text_single_newline_as_space,
    )))
    .context("")
    .parse(source)?;
    Ok((source, results))
}

pub fn plain_text_string_base(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, result) = is_not(" \t\r\n~`*^![]{}<>|:_\\")
        .context("")
        .parse(source)?;
    Ok((source, result))
}

pub fn plain_text_string_with_line_spaces(
    source: &str,
) -> IResult<&str, Vec<&str>, ErrorTree<&str>> {
    let (source, results) = many1(alt((plain_text_string_base, space1)))
        .context("")
        .parse(source)?;
    Ok((source, results))
}

pub fn plain_text_single_newline_as_space(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, _) = pair(tag("\n"), not(tag("\n"))).context("").parse(source)?;
    Ok((source, " "))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("alfa", &["alfa"], "")]
    #[case("alfa bravo", &["alfa", " ", "bravo"], "")]
    #[case("alfa bravo\ncharlie", &["alfa", " ", "bravo", " ", "charlie"], "")]
    #[case("alfa bravo\n\ncharlie", &["alfa", " ", "bravo"], "\n\ncharlie")]
    //#[case("  ", "  ", "")]
    fn solo_plain_text_string_with_line_spaces_and_single_newlines(
        #[case] source: &str,
        #[case] got: &[&str],
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, got.to_vec());
        let parsed = plain_text_string_with_line_spaces_and_single_newlines(source).unwrap();
        assert_eq!(matcher, parsed);
    }

    #[rstest]
    #[case("alfa", "alfa", "")]
    #[case("alfa bravo", "alfa", " bravo")]
    #[case("alfa\nbravo", "alfa", "\nbravo")]
    fn solo_plain_text_string_base_tests(
        #[case] source: &str,
        #[case] got: &str,
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, got);
        let parsed = plain_text_string_base(source).unwrap();
        assert_eq!(matcher, parsed);
    }

    #[rstest]
    #[case("alfa", &["alfa"], "")]
    #[case("alfa bravo", &["alfa", " ", "bravo"], "")]
    #[case("alfa bravo\ncharlie", &["alfa", " ", "bravo"], "\ncharlie")]
    fn solo_plain_text_string_with_line_spaces_tests(
        #[case] source: &str,
        #[case] got: &[&str],
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, got.to_vec());
        let parsed = plain_text_string_with_line_spaces(source).unwrap();
        assert_eq!(matcher, parsed);
    }

    #[rstest]
    #[case("\n", " ", "")]
    fn solo_plain_text_single_newline_as_space(
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
    fn solo_invalid_plain_text_single_newline_as_space(#[case] source: &str) {
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
}
