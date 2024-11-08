use nom::bytes::complete::is_not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn plain_text_string_base(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, result) = is_not(" \t\r\n~`*^![]{}<>|:_\\")
        .context("")
        .parse(source)?;
    Ok((source, result))
}

pub fn plain_text_string_with_line_spaces(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    Ok((source, ""))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("alfa\nbravo", "alfa", "\nbravo")]
    #[case("alfa bravo", "alfa", " bravo")]
    fn solo_plain_text_string_base_tests(
        #[case] source: &str,
        #[case] got: &str,
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, got);
        let parsed = plain_text_string_base(source).unwrap();
        assert_eq!(matcher, parsed);
    }

    // #[rstest]
    // #[case("alfa\nbravo", "alfa", "\nbravo")]
    // fn solo_plain_text_string_with_line_spaces_tests(
    //     #[case] source: &str,
    //     #[case] got: &str,
    //     #[case] remainder: &str,
    // ) {
    //     let matcher = (remainder, got);
    //     let parsed = plain_text_string_with_line_spaces(source).unwrap();
    //     assert_eq!(matcher, parsed);
    // }

    //
}
